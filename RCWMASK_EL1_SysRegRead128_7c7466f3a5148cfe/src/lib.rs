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
use u_get_SCR_EL3_Type_D128En::*;
use Halted::*;
use u_get_SCR_EL3_Type_RCWMASKEn::*;
use IsFeatureImplemented::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use u_get_HCRX_EL2_Type_D128En::*;
use X_set::*;
use IsHCRXEL2Enabled::*;
use u_get_HFGRTR_EL2_Type_nRCWMASK_EL1::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use EDSCR_read::*;
use common::*;
pub fn RCWMASK_EL1_SysRegRead128_7c7466f3a5148cfe<T: Tracer>(
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
        gs_137130: bool,
        gs_137153: bool,
        u__HCRX_EL2_D128En: bool,
        gs_137126: bool,
        gs_137103: bool,
        gs_137139: bool,
        gs_137123: bool,
        gs_137124: bool,
        gs_137127: bool,
        ga_238988: ProductTypeb78df3ce1505b121,
        u__SCR_EL3_RCWMASKEn: bool,
        gs_137136: bool,
        gs_137109: bool,
        gs_137140: bool,
        u__PSTATE_EL: u8,
        gs_137111: bool,
        gs_137107: bool,
        gs_137133: bool,
        u__HFGRTR_EL2_nRCWMASK_EL1: bool,
        gs_137108: bool,
        u__SCR_EL3_D128En: bool,
        u__EDSCR_SDD: bool,
        gs_137102: bool,
        gs_137128: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_137138: bool,
        gs_137152: bool,
        gs_137131: bool,
        gs_137135: bool,
        gs_137125: bool,
        gs_137105: bool,
        gs_137104: bool,
        ga_238997: ProductTypeb78df3ce1505b121,
        gs_137132: bool,
        gs_137134: bool,
        ga_238955: ProductTypeb78df3ce1505b121,
        gs_137110: bool,
        gs_137106: bool,
        gs_137137: bool,
        gs_137129: bool,
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
        // D s_0_9: call _get_SCR_EL3_Type_RCWMASKEn(s_0_8)
        let s_0_9: bool = u_get_SCR_EL3_Type_RCWMASKEn(state, tracer, s_0_8);
        // D s_0_10: write-var __SCR_EL3_RCWMASKEn <= s_0_9
        fn_state.u__SCR_EL3_RCWMASKEn = s_0_9;
        // C s_0_11: const #90704u : u32
        let s_0_11: u32 = 90704;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_SCR_EL3_Type_D128En(s_0_12)
        let s_0_13: bool = u_get_SCR_EL3_Type_D128En(state, tracer, s_0_12);
        // D s_0_14: write-var __SCR_EL3_D128En <= s_0_13
        fn_state.u__SCR_EL3_D128En = s_0_13;
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
        // D s_0_21: call _get_HFGRTR_EL2_Type_nRCWMASK_EL1(s_0_20)
        let s_0_21: bool = u_get_HFGRTR_EL2_Type_nRCWMASK_EL1(state, tracer, s_0_20);
        // D s_0_22: write-var __HFGRTR_EL2_nRCWMASK_EL1 <= s_0_21
        fn_state.u__HFGRTR_EL2_nRCWMASK_EL1 = s_0_21;
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
        // N s_0_33: branch s_0_32 b126 b1
        if s_0_32 {
            return block_126(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b55 b2
        if s_1_5 {
            return block_55(state, tracer, fn_state);
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
        // C s_5_0: const #23616u : u32
        let s_5_0: u32 = 23616;
        // D s_5_1: read-reg s_5_0:u128
        let s_5_1: u128 = {
            let value = state.read_register::<u128>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // C s_5_2: const #64s : i
        let s_5_2: i128 = 64;
        // D s_5_3: cast zx s_5_1 -> bv
        let s_5_3: Bits = Bits::new(s_5_1 as u128, 128u16);
        // C s_5_4: const #1s : i64
        let s_5_4: i64 = 1;
        // C s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // C s_5_6: const #63s : i
        let s_5_6: i128 = 63;
        // C s_5_7: add s_5_6 s_5_5
        let s_5_7: i128 = (s_5_6 + s_5_5);
        // D s_5_8: bit-extract s_5_3 s_5_2 s_5_7
        let s_5_8: Bits = (Bits::new(
            ((s_5_3) >> (s_5_2)).value(),
            u16::try_from(s_5_7).unwrap(),
        ));
        // D s_5_9: cast reint s_5_8 -> u64
        let s_5_9: u64 = (s_5_8.value() as u64);
        // C s_5_10: const #23616u : u32
        let s_5_10: u32 = 23616;
        // D s_5_11: read-reg s_5_10:u128
        let s_5_11: u128 = {
            let value = state.read_register::<u128>(s_5_10 as isize);
            tracer.read_register(s_5_10 as isize, value);
            value
        };
        // C s_5_12: const #0s : i
        let s_5_12: i128 = 0;
        // D s_5_13: cast zx s_5_11 -> bv
        let s_5_13: Bits = Bits::new(s_5_11 as u128, 128u16);
        // C s_5_14: const #1s : i64
        let s_5_14: i64 = 1;
        // C s_5_15: cast zx s_5_14 -> i
        let s_5_15: i128 = (i128::try_from(s_5_14).unwrap());
        // C s_5_16: const #63s : i
        let s_5_16: i128 = 63;
        // C s_5_17: add s_5_16 s_5_15
        let s_5_17: i128 = (s_5_16 + s_5_15);
        // D s_5_18: bit-extract s_5_13 s_5_12 s_5_17
        let s_5_18: Bits = (Bits::new(
            ((s_5_13) >> (s_5_12)).value(),
            u16::try_from(s_5_17).unwrap(),
        ));
        // D s_5_19: cast reint s_5_18 -> u64
        let s_5_19: u64 = (s_5_18.value() as u64);
        // D s_5_20: create-product struct = ["s_5_9", "s_5_19"]
        let s_5_20: ProductTypeb78df3ce1505b121 = ProductTypeb78df3ce1505b121 {
            _0: s_5_9,
            _1: s_5_19,
        };
        // D s_5_21: write-var ga#238997 <= s_5_20
        fn_state.ga_238997 = s_5_20;
        // D s_5_22: read-var ga#238997.0:struct
        let s_5_22: u64 = fn_state.ga_238997._0;
        // D s_5_23: read-var ga#238997.1:struct
        let s_5_23: u64 = fn_state.ga_238997._1;
        // C s_5_24: const #1s : i
        let s_5_24: i128 = 1;
        // D s_5_25: read-var t:i
        let s_5_25: i128 = fn_state.t;
        // D s_5_26: add s_5_25 s_5_24
        let s_5_26: i128 = (s_5_25 + s_5_24);
        // C s_5_27: const #64s : i64
        let s_5_27: i64 = 64;
        // D s_5_28: cast zx s_5_22 -> bv
        let s_5_28: Bits = Bits::new(s_5_22 as u128, 64u16);
        // D s_5_29: call X_set(s_5_26, s_5_27, s_5_28)
        let s_5_29: () = X_set(state, tracer, s_5_26, s_5_27, s_5_28);
        // C s_5_30: const #64s : i64
        let s_5_30: i64 = 64;
        // D s_5_31: cast zx s_5_23 -> bv
        let s_5_31: Bits = Bits::new(s_5_23 as u128, 64u16);
        // D s_5_32: read-var t:i
        let s_5_32: i128 = fn_state.t;
        // D s_5_33: call X_set(s_5_32, s_5_30, s_5_31)
        let s_5_33: () = X_set(state, tracer, s_5_32, s_5_30, s_5_31);
        // N s_5_34: return
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
        // N s_6_2: branch s_6_1 b54 b7
        if s_6_1 {
            return block_54(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#137102 <= s_7_0
        fn_state.gs_137102 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#137102:u8
        let s_8_0: bool = fn_state.gs_137102;
        // N s_8_1: branch s_8_0 b53 b9
        if s_8_0 {
            return block_53(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#137103 <= s_9_0
        fn_state.gs_137103 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#137103:u8
        let s_10_0: bool = fn_state.gs_137103;
        // N s_10_1: branch s_10_0 b52 b11
        if s_10_0 {
            return block_52(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#137104 <= s_11_0
        fn_state.gs_137104 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#137104:u8
        let s_12_0: bool = fn_state.gs_137104;
        // N s_12_1: branch s_12_0 b51 b13
        if s_12_0 {
            return block_51(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#137105 <= s_13_0
        fn_state.gs_137105 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#137105:u8
        let s_14_0: bool = fn_state.gs_137105;
        // N s_14_1: branch s_14_0 b50 b15
        if s_14_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
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
        // N s_15_2: branch s_15_1 b49 b16
        if s_15_1 {
            return block_49(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#137106 <= s_16_0
        fn_state.gs_137106 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#137106:u8
        let s_17_0: bool = fn_state.gs_137106;
        // N s_17_1: branch s_17_0 b48 b18
        if s_17_0 {
            return block_48(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#137107 <= s_18_0
        fn_state.gs_137107 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#137107:u8
        let s_19_0: bool = fn_state.gs_137107;
        // N s_19_1: branch s_19_0 b47 b20
        if s_19_0 {
            return block_47(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#137108 <= s_20_0
        fn_state.gs_137108 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#137108:u8
        let s_21_0: bool = fn_state.gs_137108;
        // N s_21_1: branch s_21_0 b46 b22
        if s_21_0 {
            return block_46(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#137109 <= s_22_0
        fn_state.gs_137109 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#137109:u8
        let s_23_0: bool = fn_state.gs_137109;
        // N s_23_1: branch s_23_0 b45 b24
        if s_23_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #424u : u32
        let s_24_0: u32 = 424;
        // D s_24_1: read-reg s_24_0:u8
        let s_24_1: u8 = {
            let value = state.read_register::<u8>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // C s_24_2: const #2u : u8
        let s_24_2: u8 = 2;
        // D s_24_3: cmp-lt s_24_1 s_24_2
        let s_24_3: bool = ((s_24_1) < (s_24_2));
        // N s_24_4: branch s_24_3 b44 b25
        if s_24_3 {
            return block_44(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#137110 <= s_25_0
        fn_state.gs_137110 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#137110:u8
        let s_26_0: bool = fn_state.gs_137110;
        // N s_26_1: branch s_26_0 b38 b27
        if s_26_0 {
            return block_38(state, tracer, fn_state);
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
        // N s_27_4: branch s_27_3 b37 b28
        if s_27_3 {
            return block_37(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#137111 <= s_28_0
        fn_state.gs_137111 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#137111:u8
        let s_29_0: bool = fn_state.gs_137111;
        // N s_29_1: branch s_29_0 b31 b30
        if s_29_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #23616u : u32
        let s_30_0: u32 = 23616;
        // D s_30_1: read-reg s_30_0:u128
        let s_30_1: u128 = {
            let value = state.read_register::<u128>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // C s_30_2: const #64s : i
        let s_30_2: i128 = 64;
        // D s_30_3: cast zx s_30_1 -> bv
        let s_30_3: Bits = Bits::new(s_30_1 as u128, 128u16);
        // C s_30_4: const #1s : i64
        let s_30_4: i64 = 1;
        // C s_30_5: cast zx s_30_4 -> i
        let s_30_5: i128 = (i128::try_from(s_30_4).unwrap());
        // C s_30_6: const #63s : i
        let s_30_6: i128 = 63;
        // C s_30_7: add s_30_6 s_30_5
        let s_30_7: i128 = (s_30_6 + s_30_5);
        // D s_30_8: bit-extract s_30_3 s_30_2 s_30_7
        let s_30_8: Bits = (Bits::new(
            ((s_30_3) >> (s_30_2)).value(),
            u16::try_from(s_30_7).unwrap(),
        ));
        // D s_30_9: cast reint s_30_8 -> u64
        let s_30_9: u64 = (s_30_8.value() as u64);
        // C s_30_10: const #23616u : u32
        let s_30_10: u32 = 23616;
        // D s_30_11: read-reg s_30_10:u128
        let s_30_11: u128 = {
            let value = state.read_register::<u128>(s_30_10 as isize);
            tracer.read_register(s_30_10 as isize, value);
            value
        };
        // C s_30_12: const #0s : i
        let s_30_12: i128 = 0;
        // D s_30_13: cast zx s_30_11 -> bv
        let s_30_13: Bits = Bits::new(s_30_11 as u128, 128u16);
        // C s_30_14: const #1s : i64
        let s_30_14: i64 = 1;
        // C s_30_15: cast zx s_30_14 -> i
        let s_30_15: i128 = (i128::try_from(s_30_14).unwrap());
        // C s_30_16: const #63s : i
        let s_30_16: i128 = 63;
        // C s_30_17: add s_30_16 s_30_15
        let s_30_17: i128 = (s_30_16 + s_30_15);
        // D s_30_18: bit-extract s_30_13 s_30_12 s_30_17
        let s_30_18: Bits = (Bits::new(
            ((s_30_13) >> (s_30_12)).value(),
            u16::try_from(s_30_17).unwrap(),
        ));
        // D s_30_19: cast reint s_30_18 -> u64
        let s_30_19: u64 = (s_30_18.value() as u64);
        // D s_30_20: create-product struct = ["s_30_9", "s_30_19"]
        let s_30_20: ProductTypeb78df3ce1505b121 = ProductTypeb78df3ce1505b121 {
            _0: s_30_9,
            _1: s_30_19,
        };
        // D s_30_21: write-var ga#238988 <= s_30_20
        fn_state.ga_238988 = s_30_20;
        // D s_30_22: read-var ga#238988.0:struct
        let s_30_22: u64 = fn_state.ga_238988._0;
        // D s_30_23: read-var ga#238988.1:struct
        let s_30_23: u64 = fn_state.ga_238988._1;
        // C s_30_24: const #1s : i
        let s_30_24: i128 = 1;
        // D s_30_25: read-var t:i
        let s_30_25: i128 = fn_state.t;
        // D s_30_26: add s_30_25 s_30_24
        let s_30_26: i128 = (s_30_25 + s_30_24);
        // C s_30_27: const #64s : i64
        let s_30_27: i64 = 64;
        // D s_30_28: cast zx s_30_22 -> bv
        let s_30_28: Bits = Bits::new(s_30_22 as u128, 64u16);
        // D s_30_29: call X_set(s_30_26, s_30_27, s_30_28)
        let s_30_29: () = X_set(state, tracer, s_30_26, s_30_27, s_30_28);
        // C s_30_30: const #64s : i64
        let s_30_30: i64 = 64;
        // D s_30_31: cast zx s_30_23 -> bv
        let s_30_31: Bits = Bits::new(s_30_23 as u128, 64u16);
        // D s_30_32: read-var t:i
        let s_30_32: i128 = fn_state.t;
        // D s_30_33: call X_set(s_30_32, s_30_30, s_30_31)
        let s_30_33: () = X_set(state, tracer, s_30_32, s_30_30, s_30_31);
        // N s_30_34: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call Halted(s_31_0)
        let s_31_1: bool = Halted(state, tracer, s_31_0);
        // N s_31_2: branch s_31_1 b36 b32
        if s_31_1 {
            return block_36(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#137123 <= s_32_0
        fn_state.gs_137123 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#137123:u8
        let s_33_0: bool = fn_state.gs_137123;
        // N s_33_1: branch s_33_0 b35 b34
        if s_33_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #20u : u8
        let s_34_0: u8 = 20;
        // C s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 8u16);
        // C s_34_2: cast zx s_34_1 -> i
        let s_34_2: i128 = (s_34_1.value() as i128);
        // C s_34_3: cast reint s_34_2 -> i64
        let s_34_3: i64 = (s_34_2 as i64);
        // C s_34_4: cast zx s_34_3 -> i
        let s_34_4: i128 = (i128::try_from(s_34_3).unwrap());
        // C s_34_5: const #424u : u32
        let s_34_5: u32 = 424;
        // D s_34_6: read-reg s_34_5:u8
        let s_34_6: u8 = {
            let value = state.read_register::<u8>(s_34_5 as isize);
            tracer.read_register(s_34_5 as isize, value);
            value
        };
        // D s_34_7: call AArch64_SystemAccessTrap(s_34_6, s_34_4)
        let s_34_7: () = AArch64_SystemAccessTrap(state, tracer, s_34_6, s_34_4);
        // N s_34_8: return
        return;
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
        // D s_36_0: read-var __EDSCR_SDD:u8
        let s_36_0: bool = fn_state.u__EDSCR_SDD;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 1u16);
        // C s_36_2: const #1u : u8
        let s_36_2: bool = true;
        // C s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 1u16);
        // D s_36_4: cmp-eq s_36_1 s_36_3
        let s_36_4: bool = ((s_36_1) == (s_36_3));
        // D s_36_5: write-var gs#137123 <= s_36_4
        fn_state.gs_137123 = s_36_4;
        // N s_36_6: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var __SCR_EL3_D128En:u8
        let s_37_0: bool = fn_state.u__SCR_EL3_D128En;
        // D s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 1u16);
        // C s_37_2: const #0u : u8
        let s_37_2: bool = false;
        // C s_37_3: cast zx s_37_2 -> bv
        let s_37_3: Bits = Bits::new(s_37_2 as u128, 1u16);
        // D s_37_4: cmp-eq s_37_1 s_37_3
        let s_37_4: bool = ((s_37_1) == (s_37_3));
        // D s_37_5: write-var gs#137111 <= s_37_4
        fn_state.gs_137111 = s_37_4;
        // N s_37_6: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #() : ()
        let s_38_0: () = ();
        // S s_38_1: call Halted(s_38_0)
        let s_38_1: bool = Halted(state, tracer, s_38_0);
        // N s_38_2: branch s_38_1 b43 b39
        if s_38_1 {
            return block_43(state, tracer, fn_state);
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
        // D s_39_1: write-var gs#137124 <= s_39_0
        fn_state.gs_137124 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#137124:u8
        let s_40_0: bool = fn_state.gs_137124;
        // N s_40_1: branch s_40_0 b42 b41
        if s_40_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #20u : u8
        let s_41_0: u8 = 20;
        // C s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 8u16);
        // C s_41_2: cast zx s_41_1 -> i
        let s_41_2: i128 = (s_41_1.value() as i128);
        // C s_41_3: cast reint s_41_2 -> i64
        let s_41_3: i64 = (s_41_2 as i64);
        // C s_41_4: cast zx s_41_3 -> i
        let s_41_4: i128 = (i128::try_from(s_41_3).unwrap());
        // C s_41_5: const #424u : u32
        let s_41_5: u32 = 424;
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
        // N s_42_0: panic
        panic!("{:?}", ());
        // N s_42_1: return
        return;
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var __EDSCR_SDD:u8
        let s_43_0: bool = fn_state.u__EDSCR_SDD;
        // D s_43_1: cast zx s_43_0 -> bv
        let s_43_1: Bits = Bits::new(s_43_0 as u128, 1u16);
        // C s_43_2: const #1u : u8
        let s_43_2: bool = true;
        // C s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 1u16);
        // D s_43_4: cmp-eq s_43_1 s_43_3
        let s_43_4: bool = ((s_43_1) == (s_43_3));
        // D s_43_5: write-var gs#137124 <= s_43_4
        fn_state.gs_137124 = s_43_4;
        // N s_43_6: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var __SCR_EL3_RCWMASKEn:u8
        let s_44_0: bool = fn_state.u__SCR_EL3_RCWMASKEn;
        // D s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 1u16);
        // C s_44_2: const #0u : u8
        let s_44_2: bool = false;
        // C s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 1u16);
        // D s_44_4: cmp-eq s_44_1 s_44_3
        let s_44_4: bool = ((s_44_1) == (s_44_3));
        // D s_44_5: write-var gs#137110 <= s_44_4
        fn_state.gs_137110 = s_44_4;
        // N s_44_6: jump b26
        return block_26(state, tracer, fn_state);
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
        // D s_46_0: read-var __SCR_EL3_D128En:u8
        let s_46_0: bool = fn_state.u__SCR_EL3_D128En;
        // D s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 1u16);
        // C s_46_2: const #0u : u8
        let s_46_2: bool = false;
        // C s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 1u16);
        // D s_46_4: cmp-eq s_46_1 s_46_3
        let s_46_4: bool = ((s_46_1) == (s_46_3));
        // D s_46_5: write-var gs#137109 <= s_46_4
        fn_state.gs_137109 = s_46_4;
        // N s_46_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_47_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_47_1: call __IMPDEF_boolean(s_47_0)
        let s_47_1: bool = u__IMPDEF_boolean(state, tracer, s_47_0);
        // D s_47_2: write-var gs#137108 <= s_47_1
        fn_state.gs_137108 = s_47_1;
        // N s_47_3: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var __EDSCR_SDD:u8
        let s_48_0: bool = fn_state.u__EDSCR_SDD;
        // D s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 1u16);
        // C s_48_2: const #1u : u8
        let s_48_2: bool = true;
        // C s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 1u16);
        // D s_48_4: cmp-eq s_48_1 s_48_3
        let s_48_4: bool = ((s_48_1) == (s_48_3));
        // D s_48_5: write-var gs#137107 <= s_48_4
        fn_state.gs_137107 = s_48_4;
        // N s_48_6: jump b19
        return block_19(state, tracer, fn_state);
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
        // D s_49_4: write-var gs#137106 <= s_49_3
        fn_state.gs_137106 = s_49_3;
        // N s_49_5: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_50_0: panic
        panic!("{:?}", ());
        // N s_50_1: return
        return;
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var __SCR_EL3_RCWMASKEn:u8
        let s_51_0: bool = fn_state.u__SCR_EL3_RCWMASKEn;
        // D s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 1u16);
        // C s_51_2: const #0u : u8
        let s_51_2: bool = false;
        // C s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 1u16);
        // D s_51_4: cmp-eq s_51_1 s_51_3
        let s_51_4: bool = ((s_51_1) == (s_51_3));
        // D s_51_5: write-var gs#137105 <= s_51_4
        fn_state.gs_137105 = s_51_4;
        // N s_51_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_52_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_52_1: call __IMPDEF_boolean(s_52_0)
        let s_52_1: bool = u__IMPDEF_boolean(state, tracer, s_52_0);
        // D s_52_2: write-var gs#137104 <= s_52_1
        fn_state.gs_137104 = s_52_1;
        // N s_52_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var __EDSCR_SDD:u8
        let s_53_0: bool = fn_state.u__EDSCR_SDD;
        // D s_53_1: cast zx s_53_0 -> bv
        let s_53_1: Bits = Bits::new(s_53_0 as u128, 1u16);
        // C s_53_2: const #1u : u8
        let s_53_2: bool = true;
        // C s_53_3: cast zx s_53_2 -> bv
        let s_53_3: Bits = Bits::new(s_53_2 as u128, 1u16);
        // D s_53_4: cmp-eq s_53_1 s_53_3
        let s_53_4: bool = ((s_53_1) == (s_53_3));
        // D s_53_5: write-var gs#137103 <= s_53_4
        fn_state.gs_137103 = s_53_4;
        // N s_53_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #424u : u32
        let s_54_0: u32 = 424;
        // D s_54_1: read-reg s_54_0:u8
        let s_54_1: u8 = {
            let value = state.read_register::<u8>(s_54_0 as isize);
            tracer.read_register(s_54_0 as isize, value);
            value
        };
        // C s_54_2: const #2u : u8
        let s_54_2: u8 = 2;
        // D s_54_3: cmp-lt s_54_1 s_54_2
        let s_54_3: bool = ((s_54_1) < (s_54_2));
        // D s_54_4: write-var gs#137102 <= s_54_3
        fn_state.gs_137102 = s_54_3;
        // N s_54_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #() : ()
        let s_55_0: () = ();
        // S s_55_1: call Halted(s_55_0)
        let s_55_1: bool = Halted(state, tracer, s_55_0);
        // N s_55_2: branch s_55_1 b125 b56
        if s_55_1 {
            return block_125(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#137125 <= s_56_0
        fn_state.gs_137125 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#137125:u8
        let s_57_0: bool = fn_state.gs_137125;
        // N s_57_1: branch s_57_0 b124 b58
        if s_57_0 {
            return block_124(state, tracer, fn_state);
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
        // D s_58_1: write-var gs#137126 <= s_58_0
        fn_state.gs_137126 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#137126:u8
        let s_59_0: bool = fn_state.gs_137126;
        // N s_59_1: branch s_59_0 b123 b60
        if s_59_0 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #0u : u8
        let s_60_0: bool = false;
        // D s_60_1: write-var gs#137127 <= s_60_0
        fn_state.gs_137127 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#137127:u8
        let s_61_0: bool = fn_state.gs_137127;
        // N s_61_1: branch s_61_0 b122 b62
        if s_61_0 {
            return block_122(state, tracer, fn_state);
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
        // D s_62_1: write-var gs#137128 <= s_62_0
        fn_state.gs_137128 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#137128:u8
        let s_63_0: bool = fn_state.gs_137128;
        // N s_63_1: branch s_63_0 b121 b64
        if s_63_0 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
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
        // N s_64_2: branch s_64_1 b120 b65
        if s_64_1 {
            return block_120(state, tracer, fn_state);
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
        // D s_65_1: write-var gs#137129 <= s_65_0
        fn_state.gs_137129 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#137129:u8
        let s_66_0: bool = fn_state.gs_137129;
        // N s_66_1: branch s_66_0 b119 b67
        if s_66_0 {
            return block_119(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #0u : u8
        let s_67_0: bool = false;
        // D s_67_1: write-var gs#137130 <= s_67_0
        fn_state.gs_137130 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var gs#137130:u8
        let s_68_0: bool = fn_state.gs_137130;
        // N s_68_1: branch s_68_0 b118 b69
        if s_68_0 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #0u : u8
        let s_69_0: bool = false;
        // D s_69_1: write-var gs#137131 <= s_69_0
        fn_state.gs_137131 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#137131:u8
        let s_70_0: bool = fn_state.gs_137131;
        // N s_70_1: branch s_70_0 b117 b71
        if s_70_0 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #0u : u8
        let s_71_0: bool = false;
        // D s_71_1: write-var gs#137132 <= s_71_0
        fn_state.gs_137132 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#137132:u8
        let s_72_0: bool = fn_state.gs_137132;
        // N s_72_1: branch s_72_0 b116 b73
        if s_72_0 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #() : ()
        let s_73_0: () = ();
        // S s_73_1: call EL2Enabled(s_73_0)
        let s_73_1: bool = EL2Enabled(state, tracer, s_73_0);
        // N s_73_2: branch s_73_1 b115 b74
        if s_73_1 {
            return block_115(state, tracer, fn_state);
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
        // D s_74_1: write-var gs#137133 <= s_74_0
        fn_state.gs_137133 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#137133:u8
        let s_75_0: bool = fn_state.gs_137133;
        // N s_75_1: branch s_75_0 b111 b76
        if s_75_0 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #0u : u8
        let s_76_0: bool = false;
        // D s_76_1: write-var gs#137135 <= s_76_0
        fn_state.gs_137135 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#137135:u8
        let s_77_0: bool = fn_state.gs_137135;
        // N s_77_1: branch s_77_0 b110 b78
        if s_77_0 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #0u : u8
        let s_78_0: bool = false;
        // D s_78_1: write-var gs#137136 <= s_78_0
        fn_state.gs_137136 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#137136:u8
        let s_79_0: bool = fn_state.gs_137136;
        // N s_79_1: branch s_79_0 b109 b80
        if s_79_0 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #() : ()
        let s_80_0: () = ();
        // S s_80_1: call EL2Enabled(s_80_0)
        let s_80_1: bool = EL2Enabled(state, tracer, s_80_0);
        // N s_80_2: branch s_80_1 b105 b81
        if s_80_1 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #0u : u8
        let s_81_0: bool = false;
        // D s_81_1: write-var gs#137138 <= s_81_0
        fn_state.gs_137138 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#137138:u8
        let s_82_0: bool = fn_state.gs_137138;
        // N s_82_1: branch s_82_0 b104 b83
        if s_82_0 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #424u : u32
        let s_83_0: u32 = 424;
        // D s_83_1: read-reg s_83_0:u8
        let s_83_1: u8 = {
            let value = state.read_register::<u8>(s_83_0 as isize);
            tracer.read_register(s_83_0 as isize, value);
            value
        };
        // C s_83_2: const #2u : u8
        let s_83_2: u8 = 2;
        // D s_83_3: cmp-lt s_83_1 s_83_2
        let s_83_3: bool = ((s_83_1) < (s_83_2));
        // N s_83_4: branch s_83_3 b103 b84
        if s_83_3 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #0u : u8
        let s_84_0: bool = false;
        // D s_84_1: write-var gs#137139 <= s_84_0
        fn_state.gs_137139 = s_84_0;
        // N s_84_2: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var gs#137139:u8
        let s_85_0: bool = fn_state.gs_137139;
        // N s_85_1: branch s_85_0 b97 b86
        if s_85_0 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #424u : u32
        let s_86_0: u32 = 424;
        // D s_86_1: read-reg s_86_0:u8
        let s_86_1: u8 = {
            let value = state.read_register::<u8>(s_86_0 as isize);
            tracer.read_register(s_86_0 as isize, value);
            value
        };
        // C s_86_2: const #2u : u8
        let s_86_2: u8 = 2;
        // D s_86_3: cmp-lt s_86_1 s_86_2
        let s_86_3: bool = ((s_86_1) < (s_86_2));
        // N s_86_4: branch s_86_3 b96 b87
        if s_86_3 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #0u : u8
        let s_87_0: bool = false;
        // D s_87_1: write-var gs#137140 <= s_87_0
        fn_state.gs_137140 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#137140:u8
        let s_88_0: bool = fn_state.gs_137140;
        // N s_88_1: branch s_88_0 b90 b89
        if s_88_0 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #23616u : u32
        let s_89_0: u32 = 23616;
        // D s_89_1: read-reg s_89_0:u128
        let s_89_1: u128 = {
            let value = state.read_register::<u128>(s_89_0 as isize);
            tracer.read_register(s_89_0 as isize, value);
            value
        };
        // C s_89_2: const #64s : i
        let s_89_2: i128 = 64;
        // D s_89_3: cast zx s_89_1 -> bv
        let s_89_3: Bits = Bits::new(s_89_1 as u128, 128u16);
        // C s_89_4: const #1s : i64
        let s_89_4: i64 = 1;
        // C s_89_5: cast zx s_89_4 -> i
        let s_89_5: i128 = (i128::try_from(s_89_4).unwrap());
        // C s_89_6: const #63s : i
        let s_89_6: i128 = 63;
        // C s_89_7: add s_89_6 s_89_5
        let s_89_7: i128 = (s_89_6 + s_89_5);
        // D s_89_8: bit-extract s_89_3 s_89_2 s_89_7
        let s_89_8: Bits = (Bits::new(
            ((s_89_3) >> (s_89_2)).value(),
            u16::try_from(s_89_7).unwrap(),
        ));
        // D s_89_9: cast reint s_89_8 -> u64
        let s_89_9: u64 = (s_89_8.value() as u64);
        // C s_89_10: const #23616u : u32
        let s_89_10: u32 = 23616;
        // D s_89_11: read-reg s_89_10:u128
        let s_89_11: u128 = {
            let value = state.read_register::<u128>(s_89_10 as isize);
            tracer.read_register(s_89_10 as isize, value);
            value
        };
        // C s_89_12: const #0s : i
        let s_89_12: i128 = 0;
        // D s_89_13: cast zx s_89_11 -> bv
        let s_89_13: Bits = Bits::new(s_89_11 as u128, 128u16);
        // C s_89_14: const #1s : i64
        let s_89_14: i64 = 1;
        // C s_89_15: cast zx s_89_14 -> i
        let s_89_15: i128 = (i128::try_from(s_89_14).unwrap());
        // C s_89_16: const #63s : i
        let s_89_16: i128 = 63;
        // C s_89_17: add s_89_16 s_89_15
        let s_89_17: i128 = (s_89_16 + s_89_15);
        // D s_89_18: bit-extract s_89_13 s_89_12 s_89_17
        let s_89_18: Bits = (Bits::new(
            ((s_89_13) >> (s_89_12)).value(),
            u16::try_from(s_89_17).unwrap(),
        ));
        // D s_89_19: cast reint s_89_18 -> u64
        let s_89_19: u64 = (s_89_18.value() as u64);
        // D s_89_20: create-product struct = ["s_89_9", "s_89_19"]
        let s_89_20: ProductTypeb78df3ce1505b121 = ProductTypeb78df3ce1505b121 {
            _0: s_89_9,
            _1: s_89_19,
        };
        // D s_89_21: write-var ga#238955 <= s_89_20
        fn_state.ga_238955 = s_89_20;
        // D s_89_22: read-var ga#238955.0:struct
        let s_89_22: u64 = fn_state.ga_238955._0;
        // D s_89_23: read-var ga#238955.1:struct
        let s_89_23: u64 = fn_state.ga_238955._1;
        // C s_89_24: const #1s : i
        let s_89_24: i128 = 1;
        // D s_89_25: read-var t:i
        let s_89_25: i128 = fn_state.t;
        // D s_89_26: add s_89_25 s_89_24
        let s_89_26: i128 = (s_89_25 + s_89_24);
        // C s_89_27: const #64s : i64
        let s_89_27: i64 = 64;
        // D s_89_28: cast zx s_89_22 -> bv
        let s_89_28: Bits = Bits::new(s_89_22 as u128, 64u16);
        // D s_89_29: call X_set(s_89_26, s_89_27, s_89_28)
        let s_89_29: () = X_set(state, tracer, s_89_26, s_89_27, s_89_28);
        // C s_89_30: const #64s : i64
        let s_89_30: i64 = 64;
        // D s_89_31: cast zx s_89_23 -> bv
        let s_89_31: Bits = Bits::new(s_89_23 as u128, 64u16);
        // D s_89_32: read-var t:i
        let s_89_32: i128 = fn_state.t;
        // D s_89_33: call X_set(s_89_32, s_89_30, s_89_31)
        let s_89_33: () = X_set(state, tracer, s_89_32, s_89_30, s_89_31);
        // N s_89_34: return
        return;
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #() : ()
        let s_90_0: () = ();
        // S s_90_1: call Halted(s_90_0)
        let s_90_1: bool = Halted(state, tracer, s_90_0);
        // N s_90_2: branch s_90_1 b95 b91
        if s_90_1 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #0u : u8
        let s_91_0: bool = false;
        // D s_91_1: write-var gs#137152 <= s_91_0
        fn_state.gs_137152 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#137152:u8
        let s_92_0: bool = fn_state.gs_137152;
        // N s_92_1: branch s_92_0 b94 b93
        if s_92_0 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #20u : u8
        let s_93_0: u8 = 20;
        // C s_93_1: cast zx s_93_0 -> bv
        let s_93_1: Bits = Bits::new(s_93_0 as u128, 8u16);
        // C s_93_2: cast zx s_93_1 -> i
        let s_93_2: i128 = (s_93_1.value() as i128);
        // C s_93_3: cast reint s_93_2 -> i64
        let s_93_3: i64 = (s_93_2 as i64);
        // C s_93_4: cast zx s_93_3 -> i
        let s_93_4: i128 = (i128::try_from(s_93_3).unwrap());
        // C s_93_5: const #424u : u32
        let s_93_5: u32 = 424;
        // D s_93_6: read-reg s_93_5:u8
        let s_93_6: u8 = {
            let value = state.read_register::<u8>(s_93_5 as isize);
            tracer.read_register(s_93_5 as isize, value);
            value
        };
        // D s_93_7: call AArch64_SystemAccessTrap(s_93_6, s_93_4)
        let s_93_7: () = AArch64_SystemAccessTrap(state, tracer, s_93_6, s_93_4);
        // N s_93_8: return
        return;
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_94_0: panic
        panic!("{:?}", ());
        // N s_94_1: return
        return;
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var __EDSCR_SDD:u8
        let s_95_0: bool = fn_state.u__EDSCR_SDD;
        // D s_95_1: cast zx s_95_0 -> bv
        let s_95_1: Bits = Bits::new(s_95_0 as u128, 1u16);
        // C s_95_2: const #1u : u8
        let s_95_2: bool = true;
        // C s_95_3: cast zx s_95_2 -> bv
        let s_95_3: Bits = Bits::new(s_95_2 as u128, 1u16);
        // D s_95_4: cmp-eq s_95_1 s_95_3
        let s_95_4: bool = ((s_95_1) == (s_95_3));
        // D s_95_5: write-var gs#137152 <= s_95_4
        fn_state.gs_137152 = s_95_4;
        // N s_95_6: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var __SCR_EL3_D128En:u8
        let s_96_0: bool = fn_state.u__SCR_EL3_D128En;
        // D s_96_1: cast zx s_96_0 -> bv
        let s_96_1: Bits = Bits::new(s_96_0 as u128, 1u16);
        // C s_96_2: const #0u : u8
        let s_96_2: bool = false;
        // C s_96_3: cast zx s_96_2 -> bv
        let s_96_3: Bits = Bits::new(s_96_2 as u128, 1u16);
        // D s_96_4: cmp-eq s_96_1 s_96_3
        let s_96_4: bool = ((s_96_1) == (s_96_3));
        // D s_96_5: write-var gs#137140 <= s_96_4
        fn_state.gs_137140 = s_96_4;
        // N s_96_6: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #() : ()
        let s_97_0: () = ();
        // S s_97_1: call Halted(s_97_0)
        let s_97_1: bool = Halted(state, tracer, s_97_0);
        // N s_97_2: branch s_97_1 b102 b98
        if s_97_1 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #0u : u8
        let s_98_0: bool = false;
        // D s_98_1: write-var gs#137153 <= s_98_0
        fn_state.gs_137153 = s_98_0;
        // N s_98_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var gs#137153:u8
        let s_99_0: bool = fn_state.gs_137153;
        // N s_99_1: branch s_99_0 b101 b100
        if s_99_0 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #20u : u8
        let s_100_0: u8 = 20;
        // C s_100_1: cast zx s_100_0 -> bv
        let s_100_1: Bits = Bits::new(s_100_0 as u128, 8u16);
        // C s_100_2: cast zx s_100_1 -> i
        let s_100_2: i128 = (s_100_1.value() as i128);
        // C s_100_3: cast reint s_100_2 -> i64
        let s_100_3: i64 = (s_100_2 as i64);
        // C s_100_4: cast zx s_100_3 -> i
        let s_100_4: i128 = (i128::try_from(s_100_3).unwrap());
        // C s_100_5: const #424u : u32
        let s_100_5: u32 = 424;
        // D s_100_6: read-reg s_100_5:u8
        let s_100_6: u8 = {
            let value = state.read_register::<u8>(s_100_5 as isize);
            tracer.read_register(s_100_5 as isize, value);
            value
        };
        // D s_100_7: call AArch64_SystemAccessTrap(s_100_6, s_100_4)
        let s_100_7: () = AArch64_SystemAccessTrap(state, tracer, s_100_6, s_100_4);
        // N s_100_8: return
        return;
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_101_0: panic
        panic!("{:?}", ());
        // N s_101_1: return
        return;
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var __EDSCR_SDD:u8
        let s_102_0: bool = fn_state.u__EDSCR_SDD;
        // D s_102_1: cast zx s_102_0 -> bv
        let s_102_1: Bits = Bits::new(s_102_0 as u128, 1u16);
        // C s_102_2: const #1u : u8
        let s_102_2: bool = true;
        // C s_102_3: cast zx s_102_2 -> bv
        let s_102_3: Bits = Bits::new(s_102_2 as u128, 1u16);
        // D s_102_4: cmp-eq s_102_1 s_102_3
        let s_102_4: bool = ((s_102_1) == (s_102_3));
        // D s_102_5: write-var gs#137153 <= s_102_4
        fn_state.gs_137153 = s_102_4;
        // N s_102_6: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var __SCR_EL3_RCWMASKEn:u8
        let s_103_0: bool = fn_state.u__SCR_EL3_RCWMASKEn;
        // D s_103_1: cast zx s_103_0 -> bv
        let s_103_1: Bits = Bits::new(s_103_0 as u128, 1u16);
        // C s_103_2: const #0u : u8
        let s_103_2: bool = false;
        // C s_103_3: cast zx s_103_2 -> bv
        let s_103_3: Bits = Bits::new(s_103_2 as u128, 1u16);
        // D s_103_4: cmp-eq s_103_1 s_103_3
        let s_103_4: bool = ((s_103_1) == (s_103_3));
        // D s_103_5: write-var gs#137139 <= s_103_4
        fn_state.gs_137139 = s_103_4;
        // N s_103_6: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #20u : u8
        let s_104_0: u8 = 20;
        // C s_104_1: cast zx s_104_0 -> bv
        let s_104_1: Bits = Bits::new(s_104_0 as u128, 8u16);
        // C s_104_2: cast zx s_104_1 -> i
        let s_104_2: i128 = (s_104_1.value() as i128);
        // C s_104_3: cast reint s_104_2 -> i64
        let s_104_3: i64 = (s_104_2 as i64);
        // C s_104_4: cast zx s_104_3 -> i
        let s_104_4: i128 = (i128::try_from(s_104_3).unwrap());
        // C s_104_5: const #432u : u32
        let s_104_5: u32 = 432;
        // D s_104_6: read-reg s_104_5:u8
        let s_104_6: u8 = {
            let value = state.read_register::<u8>(s_104_5 as isize);
            tracer.read_register(s_104_5 as isize, value);
            value
        };
        // D s_104_7: call AArch64_SystemAccessTrap(s_104_6, s_104_4)
        let s_104_7: () = AArch64_SystemAccessTrap(state, tracer, s_104_6, s_104_4);
        // N s_104_8: return
        return;
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #() : ()
        let s_105_0: () = ();
        // S s_105_1: call IsHCRXEL2Enabled(s_105_0)
        let s_105_1: bool = IsHCRXEL2Enabled(state, tracer, s_105_0);
        // S s_105_2: not s_105_1
        let s_105_2: bool = !s_105_1;
        // N s_105_3: branch s_105_2 b108 b106
        if s_105_2 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var __HCRX_EL2_D128En:u8
        let s_106_0: bool = fn_state.u__HCRX_EL2_D128En;
        // D s_106_1: cast zx s_106_0 -> bv
        let s_106_1: Bits = Bits::new(s_106_0 as u128, 1u16);
        // C s_106_2: const #0u : u8
        let s_106_2: bool = false;
        // C s_106_3: cast zx s_106_2 -> bv
        let s_106_3: Bits = Bits::new(s_106_2 as u128, 1u16);
        // D s_106_4: cmp-eq s_106_1 s_106_3
        let s_106_4: bool = ((s_106_1) == (s_106_3));
        // D s_106_5: write-var gs#137137 <= s_106_4
        fn_state.gs_137137 = s_106_4;
        // N s_106_6: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var gs#137137:u8
        let s_107_0: bool = fn_state.gs_137137;
        // D s_107_1: write-var gs#137138 <= s_107_0
        fn_state.gs_137138 = s_107_0;
        // N s_107_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #1u : u8
        let s_108_0: bool = true;
        // D s_108_1: write-var gs#137137 <= s_108_0
        fn_state.gs_137137 = s_108_0;
        // N s_108_2: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #20u : u8
        let s_109_0: u8 = 20;
        // C s_109_1: cast zx s_109_0 -> bv
        let s_109_1: Bits = Bits::new(s_109_0 as u128, 8u16);
        // C s_109_2: cast zx s_109_1 -> i
        let s_109_2: i128 = (s_109_1.value() as i128);
        // C s_109_3: cast reint s_109_2 -> i64
        let s_109_3: i64 = (s_109_2 as i64);
        // C s_109_4: cast zx s_109_3 -> i
        let s_109_4: i128 = (i128::try_from(s_109_3).unwrap());
        // C s_109_5: const #432u : u32
        let s_109_5: u32 = 432;
        // D s_109_6: read-reg s_109_5:u8
        let s_109_6: u8 = {
            let value = state.read_register::<u8>(s_109_5 as isize);
            tracer.read_register(s_109_5 as isize, value);
            value
        };
        // D s_109_7: call AArch64_SystemAccessTrap(s_109_6, s_109_4)
        let s_109_7: () = AArch64_SystemAccessTrap(state, tracer, s_109_6, s_109_4);
        // N s_109_8: return
        return;
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var __HFGRTR_EL2_nRCWMASK_EL1:u8
        let s_110_0: bool = fn_state.u__HFGRTR_EL2_nRCWMASK_EL1;
        // D s_110_1: cast zx s_110_0 -> bv
        let s_110_1: Bits = Bits::new(s_110_0 as u128, 1u16);
        // C s_110_2: const #0u : u8
        let s_110_2: bool = false;
        // C s_110_3: cast zx s_110_2 -> bv
        let s_110_3: Bits = Bits::new(s_110_2 as u128, 1u16);
        // D s_110_4: cmp-eq s_110_1 s_110_3
        let s_110_4: bool = ((s_110_1) == (s_110_3));
        // D s_110_5: write-var gs#137136 <= s_110_4
        fn_state.gs_137136 = s_110_4;
        // N s_110_6: jump b79
        return block_79(state, tracer, fn_state);
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
        // D s_111_4: not s_111_3
        let s_111_4: bool = !s_111_3;
        // N s_111_5: branch s_111_4 b114 b112
        if s_111_4 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var __SCR_EL3_FGTEn:u8
        let s_112_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_112_1: cast zx s_112_0 -> bv
        let s_112_1: Bits = Bits::new(s_112_0 as u128, 1u16);
        // C s_112_2: const #1u : u8
        let s_112_2: bool = true;
        // C s_112_3: cast zx s_112_2 -> bv
        let s_112_3: Bits = Bits::new(s_112_2 as u128, 1u16);
        // D s_112_4: cmp-eq s_112_1 s_112_3
        let s_112_4: bool = ((s_112_1) == (s_112_3));
        // D s_112_5: write-var gs#137134 <= s_112_4
        fn_state.gs_137134 = s_112_4;
        // N s_112_6: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#137134:u8
        let s_113_0: bool = fn_state.gs_137134;
        // D s_113_1: write-var gs#137135 <= s_113_0
        fn_state.gs_137135 = s_113_0;
        // N s_113_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #1u : u8
        let s_114_0: bool = true;
        // D s_114_1: write-var gs#137134 <= s_114_0
        fn_state.gs_137134 = s_114_0;
        // N s_114_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #146u : u32
        let s_115_0: u32 = 146;
        // S s_115_1: call IsFeatureImplemented(s_115_0)
        let s_115_1: bool = IsFeatureImplemented(state, tracer, s_115_0);
        // D s_115_2: write-var gs#137133 <= s_115_1
        fn_state.gs_137133 = s_115_1;
        // N s_115_3: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_116_0: panic
        panic!("{:?}", ());
        // N s_116_1: return
        return;
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var __SCR_EL3_D128En:u8
        let s_117_0: bool = fn_state.u__SCR_EL3_D128En;
        // D s_117_1: cast zx s_117_0 -> bv
        let s_117_1: Bits = Bits::new(s_117_0 as u128, 1u16);
        // C s_117_2: const #0u : u8
        let s_117_2: bool = false;
        // C s_117_3: cast zx s_117_2 -> bv
        let s_117_3: Bits = Bits::new(s_117_2 as u128, 1u16);
        // D s_117_4: cmp-eq s_117_1 s_117_3
        let s_117_4: bool = ((s_117_1) == (s_117_3));
        // D s_117_5: write-var gs#137132 <= s_117_4
        fn_state.gs_137132 = s_117_4;
        // N s_117_6: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_118_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_118_1: call __IMPDEF_boolean(s_118_0)
        let s_118_1: bool = u__IMPDEF_boolean(state, tracer, s_118_0);
        // D s_118_2: write-var gs#137131 <= s_118_1
        fn_state.gs_137131 = s_118_1;
        // N s_118_3: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var __EDSCR_SDD:u8
        let s_119_0: bool = fn_state.u__EDSCR_SDD;
        // D s_119_1: cast zx s_119_0 -> bv
        let s_119_1: Bits = Bits::new(s_119_0 as u128, 1u16);
        // C s_119_2: const #1u : u8
        let s_119_2: bool = true;
        // C s_119_3: cast zx s_119_2 -> bv
        let s_119_3: Bits = Bits::new(s_119_2 as u128, 1u16);
        // D s_119_4: cmp-eq s_119_1 s_119_3
        let s_119_4: bool = ((s_119_1) == (s_119_3));
        // D s_119_5: write-var gs#137130 <= s_119_4
        fn_state.gs_137130 = s_119_4;
        // N s_119_6: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #424u : u32
        let s_120_0: u32 = 424;
        // D s_120_1: read-reg s_120_0:u8
        let s_120_1: u8 = {
            let value = state.read_register::<u8>(s_120_0 as isize);
            tracer.read_register(s_120_0 as isize, value);
            value
        };
        // C s_120_2: const #2u : u8
        let s_120_2: u8 = 2;
        // D s_120_3: cmp-lt s_120_1 s_120_2
        let s_120_3: bool = ((s_120_1) < (s_120_2));
        // D s_120_4: write-var gs#137129 <= s_120_3
        fn_state.gs_137129 = s_120_3;
        // N s_120_5: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_121_0: panic
        panic!("{:?}", ());
        // N s_121_1: return
        return;
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var __SCR_EL3_RCWMASKEn:u8
        let s_122_0: bool = fn_state.u__SCR_EL3_RCWMASKEn;
        // D s_122_1: cast zx s_122_0 -> bv
        let s_122_1: Bits = Bits::new(s_122_0 as u128, 1u16);
        // C s_122_2: const #0u : u8
        let s_122_2: bool = false;
        // C s_122_3: cast zx s_122_2 -> bv
        let s_122_3: Bits = Bits::new(s_122_2 as u128, 1u16);
        // D s_122_4: cmp-eq s_122_1 s_122_3
        let s_122_4: bool = ((s_122_1) == (s_122_3));
        // D s_122_5: write-var gs#137128 <= s_122_4
        fn_state.gs_137128 = s_122_4;
        // N s_122_6: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_123_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_123_1: call __IMPDEF_boolean(s_123_0)
        let s_123_1: bool = u__IMPDEF_boolean(state, tracer, s_123_0);
        // D s_123_2: write-var gs#137127 <= s_123_1
        fn_state.gs_137127 = s_123_1;
        // N s_123_3: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var __EDSCR_SDD:u8
        let s_124_0: bool = fn_state.u__EDSCR_SDD;
        // D s_124_1: cast zx s_124_0 -> bv
        let s_124_1: Bits = Bits::new(s_124_0 as u128, 1u16);
        // C s_124_2: const #1u : u8
        let s_124_2: bool = true;
        // C s_124_3: cast zx s_124_2 -> bv
        let s_124_3: Bits = Bits::new(s_124_2 as u128, 1u16);
        // D s_124_4: cmp-eq s_124_1 s_124_3
        let s_124_4: bool = ((s_124_1) == (s_124_3));
        // D s_124_5: write-var gs#137126 <= s_124_4
        fn_state.gs_137126 = s_124_4;
        // N s_124_6: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #424u : u32
        let s_125_0: u32 = 424;
        // D s_125_1: read-reg s_125_0:u8
        let s_125_1: u8 = {
            let value = state.read_register::<u8>(s_125_0 as isize);
            tracer.read_register(s_125_0 as isize, value);
            value
        };
        // C s_125_2: const #2u : u8
        let s_125_2: u8 = 2;
        // D s_125_3: cmp-lt s_125_1 s_125_2
        let s_125_3: bool = ((s_125_1) < (s_125_2));
        // D s_125_4: write-var gs#137125 <= s_125_3
        fn_state.gs_137125 = s_125_3;
        // N s_125_5: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_126_0: panic
        panic!("{:?}", ());
        // N s_126_1: return
        return;
    }
}
