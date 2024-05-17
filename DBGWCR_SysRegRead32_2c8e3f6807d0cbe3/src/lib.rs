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
use u_get_MDCR_EL2_Type_TDA::*;
use Halted::*;
use DBGWCR_read::*;
use HDCR_read::*;
use u_get_DBGOSLSR_Type_OSLK::*;
use u_get_HDCR_Type_TDE::*;
use u__IMPDEF_boolean::*;
use u_get_MDCR_EL2_Type_TDE::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_HDCR_Type_TDA::*;
use HaltingAllowed::*;
use u_get_MDCR_EL3_Type_TDA::*;
use ELUsingAArch32::*;
use R_set::*;
use Halt::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_EDSCR_Type_TDA::*;
use DBGOSLSR_read::*;
use EL2Enabled::*;
use u__get_DBGWCR::*;
use EDSCR_read::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn DBGWCR_SysRegRead32_2c8e3f6807d0cbe3<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    coproc: u8,
    opc1: u8,
    CRn: u8,
    opc2: u8,
    CRm: u8,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_109471: bool,
        gs_109460: bool,
        gs_109472: bool,
        gs_109474: bool,
        gs_109470: bool,
        gs_109456: bool,
        gs_109469: bool,
        gs_109477: bool,
        gs_109453: bool,
        gs_109478: bool,
        u__MDCR_EL3_TDA: bool,
        gs_109473: bool,
        gs_109467: bool,
        gs_109479: bool,
        ga_173749: ProductType700c18a878c5601b,
        gs_109462: bool,
        gs_109463: bool,
        gs_109476: bool,
        gs_109458: bool,
        gs_109475: bool,
        gs_109466: bool,
        gs_109454: bool,
        gs_109468: bool,
        gs_109457: bool,
        gs_109481: bool,
        gs_109459: bool,
        ga_173787: ProductType700c18a878c5601b,
        gs_109464: bool,
        ga_173778: ProductType700c18a878c5601b,
        u__PSTATE_EL: u8,
        u__EDSCR_TDA: bool,
        gs_109461: bool,
        el: u8,
        coproc: u8,
        opc1: u8,
        CRn: u8,
        opc2: u8,
        CRm: u8,
        t: i128,
    }
    let fn_state = FunctionState {
        el,
        coproc,
        opc1,
        CRn,
        opc2,
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
        // C s_0_3: const #22712u : u32
        let s_0_3: u32 = 22712;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_MDCR_EL3_Type_TDA(s_0_4)
        let s_0_5: bool = u_get_MDCR_EL3_Type_TDA(state, tracer, s_0_4);
        // D s_0_6: write-var __MDCR_EL3_TDA <= s_0_5
        fn_state.u__MDCR_EL3_TDA = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call EDSCR_read(s_0_7)
        let s_0_8: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_0_7);
        // S s_0_9: call _get_EDSCR_Type_TDA(s_0_8)
        let s_0_9: bool = u_get_EDSCR_Type_TDA(state, tracer, s_0_8);
        // D s_0_10: write-var __EDSCR_TDA <= s_0_9
        fn_state.u__EDSCR_TDA = s_0_9;
        // C s_0_11: const #9s : i
        let s_0_11: i128 = 9;
        // C s_0_12: const #19360u : u32
        let s_0_12: u32 = 19360;
        // D s_0_13: read-reg s_0_12:i
        let s_0_13: i128 = {
            let value = state.read_register::<i128>(s_0_12 as isize);
            tracer.read_register(s_0_12 as isize, value);
            value
        };
        // D s_0_14: cmp-ge s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) >= (s_0_13));
        // N s_0_15: branch s_0_14 b110 b1
        if s_0_14 {
            return block_110(state, tracer, fn_state);
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
        // C s_1_2: const #448u : u32
        let s_1_2: u32 = 448;
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
        // N s_1_6: branch s_1_5 b109 b2
        if s_1_5 {
            return block_109(state, tracer, fn_state);
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
        // C s_2_2: const #440u : u32
        let s_2_2: u32 = 440;
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
        // N s_2_6: branch s_2_5 b54 b3
        if s_2_5 {
            return block_54(state, tracer, fn_state);
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
        // C s_3_2: const #432u : u32
        let s_3_2: u32 = 432;
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
        // N s_3_6: branch s_3_5 b15 b4
        if s_3_5 {
            return block_15(state, tracer, fn_state);
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
        // C s_4_2: const #424u : u32
        let s_4_2: u32 = 424;
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
        // N s_4_6: branch s_4_5 b6 b5
        if s_4_5 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call DBGOSLSR_read(s_6_0)
        let s_6_1: ProductType700c18a878c5601b = DBGOSLSR_read(state, tracer, s_6_0);
        // S s_6_2: call _get_DBGOSLSR_Type_OSLK(s_6_1)
        let s_6_2: bool = u_get_DBGOSLSR_Type_OSLK(state, tracer, s_6_1);
        // S s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // C s_6_4: const #0u : u8
        let s_6_4: bool = false;
        // C s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 1u16);
        // S s_6_6: cmp-eq s_6_3 s_6_5
        let s_6_6: bool = ((s_6_3) == (s_6_5));
        // N s_6_7: branch s_6_6 b14 b7
        if s_6_6 {
            return block_14(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#109453 <= s_7_0
        fn_state.gs_109453 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#109453:u8
        let s_8_0: bool = fn_state.gs_109453;
        // N s_8_1: branch s_8_0 b13 b9
        if s_8_0 {
            return block_13(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#109454 <= s_9_0
        fn_state.gs_109454 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#109454:u8
        let s_10_0: bool = fn_state.gs_109454;
        // N s_10_1: branch s_10_0 b12 b11
        if s_10_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #9s : i64
        let s_11_0: i64 = 9;
        // S s_11_1: call DBGWCR_read(s_11_0)
        let s_11_1: ProductType700c18a878c5601b = DBGWCR_read(state, tracer, s_11_0);
        // S s_11_2: call __get_DBGWCR(s_11_1)
        let s_11_2: ProductType700c18a878c5601b = u__get_DBGWCR(state, tracer, s_11_1);
        // D s_11_3: write-var ga#173787 <= s_11_2
        fn_state.ga_173787 = s_11_2;
        // D s_11_4: read-var ga#173787.0:struct
        let s_11_4: u32 = fn_state.ga_173787._0;
        // D s_11_5: read-var t:i
        let s_11_5: i128 = fn_state.t;
        // D s_11_6: call R_set(s_11_5, s_11_4)
        let s_11_6: () = R_set(state, tracer, s_11_5, s_11_4);
        // N s_11_7: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1160u : u32
        let s_12_0: u32 = 1160;
        // D s_12_1: read-reg s_12_0:u8
        let s_12_1: u8 = {
            let value = state.read_register::<u8>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: call Halt(s_12_1)
        let s_12_2: () = Halt(state, tracer, s_12_1);
        // N s_12_3: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var __EDSCR_TDA:u8
        let s_13_0: bool = fn_state.u__EDSCR_TDA;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 1u16);
        // C s_13_2: const #1u : u8
        let s_13_2: bool = true;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // D s_13_5: write-var gs#109454 <= s_13_4
        fn_state.gs_109454 = s_13_4;
        // N s_13_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call HaltingAllowed(s_14_0)
        let s_14_1: bool = HaltingAllowed(state, tracer, s_14_0);
        // D s_14_2: write-var gs#109453 <= s_14_1
        fn_state.gs_109453 = s_14_1;
        // N s_14_3: jump b8
        return block_8(state, tracer, fn_state);
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
        // N s_15_2: branch s_15_1 b53 b16
        if s_15_1 {
            return block_53(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#109456 <= s_16_0
        fn_state.gs_109456 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#109456:u8
        let s_17_0: bool = fn_state.gs_109456;
        // N s_17_1: branch s_17_0 b52 b18
        if s_17_0 {
            return block_52(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#109457 <= s_18_0
        fn_state.gs_109457 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#109457:u8
        let s_19_0: bool = fn_state.gs_109457;
        // N s_19_1: branch s_19_0 b51 b20
        if s_19_0 {
            return block_51(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#109458 <= s_20_0
        fn_state.gs_109458 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#109458:u8
        let s_21_0: bool = fn_state.gs_109458;
        // N s_21_1: branch s_21_0 b50 b22
        if s_21_0 {
            return block_50(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#109459 <= s_22_0
        fn_state.gs_109459 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#109459:u8
        let s_23_0: bool = fn_state.gs_109459;
        // N s_23_1: branch s_23_0 b49 b24
        if s_23_0 {
            return block_49(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#109460 <= s_24_0
        fn_state.gs_109460 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#109460:u8
        let s_25_0: bool = fn_state.gs_109460;
        // N s_25_1: branch s_25_0 b48 b26
        if s_25_0 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #424u : u32
        let s_26_0: u32 = 424;
        // D s_26_1: read-reg s_26_0:u8
        let s_26_1: u8 = {
            let value = state.read_register::<u8>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // C s_26_2: const #2u : u8
        let s_26_2: u8 = 2;
        // D s_26_3: cmp-lt s_26_1 s_26_2
        let s_26_3: bool = ((s_26_1) < (s_26_2));
        // N s_26_4: branch s_26_3 b47 b27
        if s_26_3 {
            return block_47(state, tracer, fn_state);
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
        // D s_27_1: write-var gs#109461 <= s_27_0
        fn_state.gs_109461 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#109461:u8
        let s_28_0: bool = fn_state.gs_109461;
        // N s_28_1: branch s_28_0 b46 b29
        if s_28_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#109462 <= s_29_0
        fn_state.gs_109462 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#109462:u8
        let s_30_0: bool = fn_state.gs_109462;
        // N s_30_1: branch s_30_0 b40 b31
        if s_30_0 {
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
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call DBGOSLSR_read(s_31_0)
        let s_31_1: ProductType700c18a878c5601b = DBGOSLSR_read(state, tracer, s_31_0);
        // S s_31_2: call _get_DBGOSLSR_Type_OSLK(s_31_1)
        let s_31_2: bool = u_get_DBGOSLSR_Type_OSLK(state, tracer, s_31_1);
        // S s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 1u16);
        // C s_31_4: const #0u : u8
        let s_31_4: bool = false;
        // C s_31_5: cast zx s_31_4 -> bv
        let s_31_5: Bits = Bits::new(s_31_4 as u128, 1u16);
        // S s_31_6: cmp-eq s_31_3 s_31_5
        let s_31_6: bool = ((s_31_3) == (s_31_5));
        // N s_31_7: branch s_31_6 b39 b32
        if s_31_6 {
            return block_39(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#109463 <= s_32_0
        fn_state.gs_109463 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#109463:u8
        let s_33_0: bool = fn_state.gs_109463;
        // N s_33_1: branch s_33_0 b38 b34
        if s_33_0 {
            return block_38(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#109464 <= s_34_0
        fn_state.gs_109464 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#109464:u8
        let s_35_0: bool = fn_state.gs_109464;
        // N s_35_1: branch s_35_0 b37 b36
        if s_35_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #9s : i64
        let s_36_0: i64 = 9;
        // S s_36_1: call DBGWCR_read(s_36_0)
        let s_36_1: ProductType700c18a878c5601b = DBGWCR_read(state, tracer, s_36_0);
        // S s_36_2: call __get_DBGWCR(s_36_1)
        let s_36_2: ProductType700c18a878c5601b = u__get_DBGWCR(state, tracer, s_36_1);
        // D s_36_3: write-var ga#173778 <= s_36_2
        fn_state.ga_173778 = s_36_2;
        // D s_36_4: read-var ga#173778.0:struct
        let s_36_4: u32 = fn_state.ga_173778._0;
        // D s_36_5: read-var t:i
        let s_36_5: i128 = fn_state.t;
        // D s_36_6: call R_set(s_36_5, s_36_4)
        let s_36_6: () = R_set(state, tracer, s_36_5, s_36_4);
        // N s_36_7: return
        return;
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #1160u : u32
        let s_37_0: u32 = 1160;
        // D s_37_1: read-reg s_37_0:u8
        let s_37_1: u8 = {
            let value = state.read_register::<u8>(s_37_0 as isize);
            tracer.read_register(s_37_0 as isize, value);
            value
        };
        // D s_37_2: call Halt(s_37_1)
        let s_37_2: () = Halt(state, tracer, s_37_1);
        // N s_37_3: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var __EDSCR_TDA:u8
        let s_38_0: bool = fn_state.u__EDSCR_TDA;
        // D s_38_1: cast zx s_38_0 -> bv
        let s_38_1: Bits = Bits::new(s_38_0 as u128, 1u16);
        // C s_38_2: const #1u : u8
        let s_38_2: bool = true;
        // C s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 1u16);
        // D s_38_4: cmp-eq s_38_1 s_38_3
        let s_38_4: bool = ((s_38_1) == (s_38_3));
        // D s_38_5: write-var gs#109464 <= s_38_4
        fn_state.gs_109464 = s_38_4;
        // N s_38_6: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #() : ()
        let s_39_0: () = ();
        // S s_39_1: call HaltingAllowed(s_39_0)
        let s_39_1: bool = HaltingAllowed(state, tracer, s_39_0);
        // D s_39_2: write-var gs#109463 <= s_39_1
        fn_state.gs_109463 = s_39_1;
        // N s_39_3: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call Halted(s_40_0)
        let s_40_1: bool = Halted(state, tracer, s_40_0);
        // N s_40_2: branch s_40_1 b45 b41
        if s_40_1 {
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
        // D s_41_1: write-var gs#109466 <= s_41_0
        fn_state.gs_109466 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#109466:u8
        let s_42_0: bool = fn_state.gs_109466;
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
        // C s_43_0: const #5u : u8
        let s_43_0: u8 = 5;
        // C s_43_1: cast zx s_43_0 -> bv
        let s_43_1: Bits = Bits::new(s_43_0 as u128, 8u16);
        // C s_43_2: cast zx s_43_1 -> i
        let s_43_2: i128 = (s_43_1.value() as i128);
        // C s_43_3: cast reint s_43_2 -> i64
        let s_43_3: i64 = (s_43_2 as i64);
        // C s_43_4: cast zx s_43_3 -> i
        let s_43_4: i128 = (i128::try_from(s_43_3).unwrap());
        // C s_43_5: const #424u : u32
        let s_43_5: u32 = 424;
        // D s_43_6: read-reg s_43_5:u8
        let s_43_6: u8 = {
            let value = state.read_register::<u8>(s_43_5 as isize);
            tracer.read_register(s_43_5 as isize, value);
            value
        };
        // D s_43_7: call AArch64_AArch32SystemAccessTrap(s_43_6, s_43_4)
        let s_43_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_43_6, s_43_4);
        // N s_43_8: return
        return;
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_44_0: panic
        panic!("{:?}", ());
        // N s_44_1: return
        return;
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call EDSCR_read(s_45_0)
        let s_45_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_45_0);
        // S s_45_2: call _get_EDSCR_Type_SDD(s_45_1)
        let s_45_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_45_1);
        // S s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 1u16);
        // C s_45_4: const #1u : u8
        let s_45_4: bool = true;
        // C s_45_5: cast zx s_45_4 -> bv
        let s_45_5: Bits = Bits::new(s_45_4 as u128, 1u16);
        // S s_45_6: cmp-eq s_45_3 s_45_5
        let s_45_6: bool = ((s_45_3) == (s_45_5));
        // D s_45_7: write-var gs#109466 <= s_45_6
        fn_state.gs_109466 = s_45_6;
        // N s_45_8: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var __MDCR_EL3_TDA:u8
        let s_46_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 1u16);
        // C s_46_2: const #1u : u8
        let s_46_2: bool = true;
        // C s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 1u16);
        // D s_46_4: cmp-eq s_46_1 s_46_3
        let s_46_4: bool = ((s_46_1) == (s_46_3));
        // D s_46_5: write-var gs#109462 <= s_46_4
        fn_state.gs_109462 = s_46_4;
        // N s_46_6: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #424u : u32
        let s_47_0: u32 = 424;
        // D s_47_1: read-reg s_47_0:u8
        let s_47_1: u8 = {
            let value = state.read_register::<u8>(s_47_0 as isize);
            tracer.read_register(s_47_0 as isize, value);
            value
        };
        // D s_47_2: call ELUsingAArch32(s_47_1)
        let s_47_2: bool = ELUsingAArch32(state, tracer, s_47_1);
        // D s_47_3: not s_47_2
        let s_47_3: bool = !s_47_2;
        // D s_47_4: write-var gs#109461 <= s_47_3
        fn_state.gs_109461 = s_47_3;
        // N s_47_5: jump b28
        return block_28(state, tracer, fn_state);
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
        // D s_49_5: write-var gs#109460 <= s_49_4
        fn_state.gs_109460 = s_49_4;
        // N s_49_6: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #424u : u32
        let s_50_0: u32 = 424;
        // D s_50_1: read-reg s_50_0:u8
        let s_50_1: u8 = {
            let value = state.read_register::<u8>(s_50_0 as isize);
            tracer.read_register(s_50_0 as isize, value);
            value
        };
        // D s_50_2: call ELUsingAArch32(s_50_1)
        let s_50_2: bool = ELUsingAArch32(state, tracer, s_50_1);
        // D s_50_3: not s_50_2
        let s_50_3: bool = !s_50_2;
        // D s_50_4: write-var gs#109459 <= s_50_3
        fn_state.gs_109459 = s_50_3;
        // N s_50_5: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_51_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_51_1: call __IMPDEF_boolean(s_51_0)
        let s_51_1: bool = u__IMPDEF_boolean(state, tracer, s_51_0);
        // D s_51_2: write-var gs#109458 <= s_51_1
        fn_state.gs_109458 = s_51_1;
        // N s_51_3: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call EDSCR_read(s_52_0)
        let s_52_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_52_0);
        // S s_52_2: call _get_EDSCR_Type_SDD(s_52_1)
        let s_52_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_52_1);
        // S s_52_3: cast zx s_52_2 -> bv
        let s_52_3: Bits = Bits::new(s_52_2 as u128, 1u16);
        // C s_52_4: const #1u : u8
        let s_52_4: bool = true;
        // C s_52_5: cast zx s_52_4 -> bv
        let s_52_5: Bits = Bits::new(s_52_4 as u128, 1u16);
        // S s_52_6: cmp-eq s_52_3 s_52_5
        let s_52_6: bool = ((s_52_3) == (s_52_5));
        // D s_52_7: write-var gs#109457 <= s_52_6
        fn_state.gs_109457 = s_52_6;
        // N s_52_8: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #424u : u32
        let s_53_0: u32 = 424;
        // D s_53_1: read-reg s_53_0:u8
        let s_53_1: u8 = {
            let value = state.read_register::<u8>(s_53_0 as isize);
            tracer.read_register(s_53_0 as isize, value);
            value
        };
        // C s_53_2: const #2u : u8
        let s_53_2: u8 = 2;
        // D s_53_3: cmp-lt s_53_1 s_53_2
        let s_53_3: bool = ((s_53_1) < (s_53_2));
        // D s_53_4: write-var gs#109456 <= s_53_3
        fn_state.gs_109456 = s_53_3;
        // N s_53_5: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #() : ()
        let s_54_0: () = ();
        // S s_54_1: call Halted(s_54_0)
        let s_54_1: bool = Halted(state, tracer, s_54_0);
        // N s_54_2: branch s_54_1 b108 b55
        if s_54_1 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #0u : u8
        let s_55_0: bool = false;
        // D s_55_1: write-var gs#109467 <= s_55_0
        fn_state.gs_109467 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#109467:u8
        let s_56_0: bool = fn_state.gs_109467;
        // N s_56_1: branch s_56_0 b107 b57
        if s_56_0 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #0u : u8
        let s_57_0: bool = false;
        // D s_57_1: write-var gs#109468 <= s_57_0
        fn_state.gs_109468 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#109468:u8
        let s_58_0: bool = fn_state.gs_109468;
        // N s_58_1: branch s_58_0 b106 b59
        if s_58_0 {
            return block_106(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#109469 <= s_59_0
        fn_state.gs_109469 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#109469:u8
        let s_60_0: bool = fn_state.gs_109469;
        // N s_60_1: branch s_60_0 b105 b61
        if s_60_0 {
            return block_105(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#109470 <= s_61_0
        fn_state.gs_109470 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#109470:u8
        let s_62_0: bool = fn_state.gs_109470;
        // N s_62_1: branch s_62_0 b104 b63
        if s_62_0 {
            return block_104(state, tracer, fn_state);
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
        // D s_63_1: write-var gs#109471 <= s_63_0
        fn_state.gs_109471 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#109471:u8
        let s_64_0: bool = fn_state.gs_109471;
        // N s_64_1: branch s_64_0 b103 b65
        if s_64_0 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #() : ()
        let s_65_0: () = ();
        // S s_65_1: call EL2Enabled(s_65_0)
        let s_65_1: bool = EL2Enabled(state, tracer, s_65_0);
        // N s_65_2: branch s_65_1 b102 b66
        if s_65_1 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #0u : u8
        let s_66_0: bool = false;
        // D s_66_1: write-var gs#109472 <= s_66_0
        fn_state.gs_109472 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#109472:u8
        let s_67_0: bool = fn_state.gs_109472;
        // N s_67_1: branch s_67_0 b101 b68
        if s_67_0 {
            return block_101(state, tracer, fn_state);
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
        // D s_68_1: write-var gs#109473 <= s_68_0
        fn_state.gs_109473 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#109473:u8
        let s_69_0: bool = fn_state.gs_109473;
        // N s_69_1: branch s_69_0 b100 b70
        if s_69_0 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #() : ()
        let s_70_0: () = ();
        // S s_70_1: call EL2Enabled(s_70_0)
        let s_70_1: bool = EL2Enabled(state, tracer, s_70_0);
        // N s_70_2: branch s_70_1 b99 b71
        if s_70_1 {
            return block_99(state, tracer, fn_state);
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
        // D s_71_1: write-var gs#109474 <= s_71_0
        fn_state.gs_109474 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#109474:u8
        let s_72_0: bool = fn_state.gs_109474;
        // N s_72_1: branch s_72_0 b98 b73
        if s_72_0 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #0u : u8
        let s_73_0: bool = false;
        // D s_73_1: write-var gs#109475 <= s_73_0
        fn_state.gs_109475 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#109475:u8
        let s_74_0: bool = fn_state.gs_109475;
        // N s_74_1: branch s_74_0 b97 b75
        if s_74_0 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #424u : u32
        let s_75_0: u32 = 424;
        // D s_75_1: read-reg s_75_0:u8
        let s_75_1: u8 = {
            let value = state.read_register::<u8>(s_75_0 as isize);
            tracer.read_register(s_75_0 as isize, value);
            value
        };
        // C s_75_2: const #2u : u8
        let s_75_2: u8 = 2;
        // D s_75_3: cmp-lt s_75_1 s_75_2
        let s_75_3: bool = ((s_75_1) < (s_75_2));
        // N s_75_4: branch s_75_3 b96 b76
        if s_75_3 {
            return block_96(state, tracer, fn_state);
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
        // D s_76_1: write-var gs#109476 <= s_76_0
        fn_state.gs_109476 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#109476:u8
        let s_77_0: bool = fn_state.gs_109476;
        // N s_77_1: branch s_77_0 b95 b78
        if s_77_0 {
            return block_95(state, tracer, fn_state);
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
        // D s_78_1: write-var gs#109477 <= s_78_0
        fn_state.gs_109477 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#109477:u8
        let s_79_0: bool = fn_state.gs_109477;
        // N s_79_1: branch s_79_0 b89 b80
        if s_79_0 {
            return block_89(state, tracer, fn_state);
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
        // S s_80_1: call DBGOSLSR_read(s_80_0)
        let s_80_1: ProductType700c18a878c5601b = DBGOSLSR_read(state, tracer, s_80_0);
        // S s_80_2: call _get_DBGOSLSR_Type_OSLK(s_80_1)
        let s_80_2: bool = u_get_DBGOSLSR_Type_OSLK(state, tracer, s_80_1);
        // S s_80_3: cast zx s_80_2 -> bv
        let s_80_3: Bits = Bits::new(s_80_2 as u128, 1u16);
        // C s_80_4: const #0u : u8
        let s_80_4: bool = false;
        // C s_80_5: cast zx s_80_4 -> bv
        let s_80_5: Bits = Bits::new(s_80_4 as u128, 1u16);
        // S s_80_6: cmp-eq s_80_3 s_80_5
        let s_80_6: bool = ((s_80_3) == (s_80_5));
        // N s_80_7: branch s_80_6 b88 b81
        if s_80_6 {
            return block_88(state, tracer, fn_state);
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
        // D s_81_1: write-var gs#109478 <= s_81_0
        fn_state.gs_109478 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#109478:u8
        let s_82_0: bool = fn_state.gs_109478;
        // N s_82_1: branch s_82_0 b87 b83
        if s_82_0 {
            return block_87(state, tracer, fn_state);
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
        // D s_83_1: write-var gs#109479 <= s_83_0
        fn_state.gs_109479 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#109479:u8
        let s_84_0: bool = fn_state.gs_109479;
        // N s_84_1: branch s_84_0 b86 b85
        if s_84_0 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #9s : i64
        let s_85_0: i64 = 9;
        // S s_85_1: call DBGWCR_read(s_85_0)
        let s_85_1: ProductType700c18a878c5601b = DBGWCR_read(state, tracer, s_85_0);
        // S s_85_2: call __get_DBGWCR(s_85_1)
        let s_85_2: ProductType700c18a878c5601b = u__get_DBGWCR(state, tracer, s_85_1);
        // D s_85_3: write-var ga#173749 <= s_85_2
        fn_state.ga_173749 = s_85_2;
        // D s_85_4: read-var ga#173749.0:struct
        let s_85_4: u32 = fn_state.ga_173749._0;
        // D s_85_5: read-var t:i
        let s_85_5: i128 = fn_state.t;
        // D s_85_6: call R_set(s_85_5, s_85_4)
        let s_85_6: () = R_set(state, tracer, s_85_5, s_85_4);
        // N s_85_7: return
        return;
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #1160u : u32
        let s_86_0: u32 = 1160;
        // D s_86_1: read-reg s_86_0:u8
        let s_86_1: u8 = {
            let value = state.read_register::<u8>(s_86_0 as isize);
            tracer.read_register(s_86_0 as isize, value);
            value
        };
        // D s_86_2: call Halt(s_86_1)
        let s_86_2: () = Halt(state, tracer, s_86_1);
        // N s_86_3: return
        return;
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var __EDSCR_TDA:u8
        let s_87_0: bool = fn_state.u__EDSCR_TDA;
        // D s_87_1: cast zx s_87_0 -> bv
        let s_87_1: Bits = Bits::new(s_87_0 as u128, 1u16);
        // C s_87_2: const #1u : u8
        let s_87_2: bool = true;
        // C s_87_3: cast zx s_87_2 -> bv
        let s_87_3: Bits = Bits::new(s_87_2 as u128, 1u16);
        // D s_87_4: cmp-eq s_87_1 s_87_3
        let s_87_4: bool = ((s_87_1) == (s_87_3));
        // D s_87_5: write-var gs#109479 <= s_87_4
        fn_state.gs_109479 = s_87_4;
        // N s_87_6: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #() : ()
        let s_88_0: () = ();
        // S s_88_1: call HaltingAllowed(s_88_0)
        let s_88_1: bool = HaltingAllowed(state, tracer, s_88_0);
        // D s_88_2: write-var gs#109478 <= s_88_1
        fn_state.gs_109478 = s_88_1;
        // N s_88_3: jump b82
        return block_82(state, tracer, fn_state);
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
        // D s_90_1: write-var gs#109481 <= s_90_0
        fn_state.gs_109481 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var gs#109481:u8
        let s_91_0: bool = fn_state.gs_109481;
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
        // C s_92_0: const #5u : u8
        let s_92_0: u8 = 5;
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
        // D s_92_7: call AArch64_AArch32SystemAccessTrap(s_92_6, s_92_4)
        let s_92_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_92_6, s_92_4);
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
        // C s_94_0: const #() : ()
        let s_94_0: () = ();
        // S s_94_1: call EDSCR_read(s_94_0)
        let s_94_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_94_0);
        // S s_94_2: call _get_EDSCR_Type_SDD(s_94_1)
        let s_94_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_94_1);
        // S s_94_3: cast zx s_94_2 -> bv
        let s_94_3: Bits = Bits::new(s_94_2 as u128, 1u16);
        // C s_94_4: const #1u : u8
        let s_94_4: bool = true;
        // C s_94_5: cast zx s_94_4 -> bv
        let s_94_5: Bits = Bits::new(s_94_4 as u128, 1u16);
        // S s_94_6: cmp-eq s_94_3 s_94_5
        let s_94_6: bool = ((s_94_3) == (s_94_5));
        // D s_94_7: write-var gs#109481 <= s_94_6
        fn_state.gs_109481 = s_94_6;
        // N s_94_8: jump b91
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
        // D s_95_5: write-var gs#109477 <= s_95_4
        fn_state.gs_109477 = s_95_4;
        // N s_95_6: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #424u : u32
        let s_96_0: u32 = 424;
        // D s_96_1: read-reg s_96_0:u8
        let s_96_1: u8 = {
            let value = state.read_register::<u8>(s_96_0 as isize);
            tracer.read_register(s_96_0 as isize, value);
            value
        };
        // D s_96_2: call ELUsingAArch32(s_96_1)
        let s_96_2: bool = ELUsingAArch32(state, tracer, s_96_1);
        // D s_96_3: not s_96_2
        let s_96_3: bool = !s_96_2;
        // D s_96_4: write-var gs#109476 <= s_96_3
        fn_state.gs_109476 = s_96_3;
        // N s_96_5: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #5u : u8
        let s_97_0: u8 = 5;
        // C s_97_1: cast zx s_97_0 -> bv
        let s_97_1: Bits = Bits::new(s_97_0 as u128, 8u16);
        // C s_97_2: cast zx s_97_1 -> i
        let s_97_2: i128 = (s_97_1.value() as i128);
        // C s_97_3: cast reint s_97_2 -> i64
        let s_97_3: i64 = (s_97_2 as i64);
        // C s_97_4: cast zx s_97_3 -> i
        let s_97_4: i128 = (i128::try_from(s_97_3).unwrap());
        // S s_97_5: call AArch32_TakeHypTrapException(s_97_4)
        let s_97_5: () = AArch32_TakeHypTrapException(state, tracer, s_97_4);
        // N s_97_6: return
        return;
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #() : ()
        let s_98_0: () = ();
        // S s_98_1: call HDCR_read(s_98_0)
        let s_98_1: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_98_0);
        // S s_98_2: call _get_HDCR_Type_TDE(s_98_1)
        let s_98_2: bool = u_get_HDCR_Type_TDE(state, tracer, s_98_1);
        // C s_98_3: const #() : ()
        let s_98_3: () = ();
        // S s_98_4: call HDCR_read(s_98_3)
        let s_98_4: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_98_3);
        // S s_98_5: call _get_HDCR_Type_TDA(s_98_4)
        let s_98_5: bool = u_get_HDCR_Type_TDA(state, tracer, s_98_4);
        // S s_98_6: cast zx s_98_2 -> bv
        let s_98_6: Bits = Bits::new(s_98_2 as u128, 1u16);
        // S s_98_7: cast zx s_98_5 -> bv
        let s_98_7: Bits = Bits::new(s_98_5 as u128, 1u16);
        // S s_98_8: cast reint s_98_6 -> u128
        let s_98_8: u128 = (s_98_6.value() as u128);
        // D s_98_9: size-of s_98_6
        let s_98_9: u16 = s_98_6.length();
        // S s_98_10: cast reint s_98_7 -> u128
        let s_98_10: u128 = (s_98_7.value() as u128);
        // D s_98_11: size-of s_98_7
        let s_98_11: u16 = s_98_7.length();
        // D s_98_12: lsl s_98_8 s_98_11
        let s_98_12: u128 = s_98_8 << s_98_11;
        // D s_98_13: or s_98_12 s_98_10
        let s_98_13: u128 = ((s_98_12) | (s_98_10));
        // D s_98_14: add s_98_9 s_98_11
        let s_98_14: u16 = (s_98_9 + s_98_11);
        // D s_98_15: create-bits s_98_13 s_98_14
        let s_98_15: Bits = Bits::new(s_98_13, s_98_14);
        // D s_98_16: cast reint s_98_15 -> u8
        let s_98_16: u8 = (s_98_15.value() as u8);
        // D s_98_17: cast zx s_98_16 -> bv
        let s_98_17: Bits = Bits::new(s_98_16 as u128, 2u16);
        // C s_98_18: const #0u : u8
        let s_98_18: u8 = 0;
        // C s_98_19: cast zx s_98_18 -> bv
        let s_98_19: Bits = Bits::new(s_98_18 as u128, 2u16);
        // D s_98_20: cmp-ne s_98_17 s_98_19
        let s_98_20: bool = ((s_98_17) != (s_98_19));
        // D s_98_21: write-var gs#109475 <= s_98_20
        fn_state.gs_109475 = s_98_20;
        // N s_98_22: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #432u : u32
        let s_99_0: u32 = 432;
        // D s_99_1: read-reg s_99_0:u8
        let s_99_1: u8 = {
            let value = state.read_register::<u8>(s_99_0 as isize);
            tracer.read_register(s_99_0 as isize, value);
            value
        };
        // D s_99_2: call ELUsingAArch32(s_99_1)
        let s_99_2: bool = ELUsingAArch32(state, tracer, s_99_1);
        // D s_99_3: write-var gs#109474 <= s_99_2
        fn_state.gs_109474 = s_99_2;
        // N s_99_4: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #5u : u8
        let s_100_0: u8 = 5;
        // C s_100_1: cast zx s_100_0 -> bv
        let s_100_1: Bits = Bits::new(s_100_0 as u128, 8u16);
        // C s_100_2: cast zx s_100_1 -> i
        let s_100_2: i128 = (s_100_1.value() as i128);
        // C s_100_3: cast reint s_100_2 -> i64
        let s_100_3: i64 = (s_100_2 as i64);
        // C s_100_4: cast zx s_100_3 -> i
        let s_100_4: i128 = (i128::try_from(s_100_3).unwrap());
        // C s_100_5: const #432u : u32
        let s_100_5: u32 = 432;
        // D s_100_6: read-reg s_100_5:u8
        let s_100_6: u8 = {
            let value = state.read_register::<u8>(s_100_5 as isize);
            tracer.read_register(s_100_5 as isize, value);
            value
        };
        // D s_100_7: call AArch64_AArch32SystemAccessTrap(s_100_6, s_100_4)
        let s_100_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_100_6,
            s_100_4,
        );
        // N s_100_8: return
        return;
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #104880u : u32
        let s_101_0: u32 = 104880;
        // D s_101_1: read-reg s_101_0:struct
        let s_101_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_101_0 as isize);
            tracer.read_register(s_101_0 as isize, value);
            value
        };
        // D s_101_2: call _get_MDCR_EL2_Type_TDE(s_101_1)
        let s_101_2: bool = u_get_MDCR_EL2_Type_TDE(state, tracer, s_101_1);
        // C s_101_3: const #104880u : u32
        let s_101_3: u32 = 104880;
        // D s_101_4: read-reg s_101_3:struct
        let s_101_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_101_3 as isize);
            tracer.read_register(s_101_3 as isize, value);
            value
        };
        // D s_101_5: call _get_MDCR_EL2_Type_TDA(s_101_4)
        let s_101_5: bool = u_get_MDCR_EL2_Type_TDA(state, tracer, s_101_4);
        // D s_101_6: cast zx s_101_2 -> bv
        let s_101_6: Bits = Bits::new(s_101_2 as u128, 1u16);
        // D s_101_7: cast zx s_101_5 -> bv
        let s_101_7: Bits = Bits::new(s_101_5 as u128, 1u16);
        // D s_101_8: cast reint s_101_6 -> u128
        let s_101_8: u128 = (s_101_6.value() as u128);
        // D s_101_9: size-of s_101_6
        let s_101_9: u16 = s_101_6.length();
        // D s_101_10: cast reint s_101_7 -> u128
        let s_101_10: u128 = (s_101_7.value() as u128);
        // D s_101_11: size-of s_101_7
        let s_101_11: u16 = s_101_7.length();
        // D s_101_12: lsl s_101_8 s_101_11
        let s_101_12: u128 = s_101_8 << s_101_11;
        // D s_101_13: or s_101_12 s_101_10
        let s_101_13: u128 = ((s_101_12) | (s_101_10));
        // D s_101_14: add s_101_9 s_101_11
        let s_101_14: u16 = (s_101_9 + s_101_11);
        // D s_101_15: create-bits s_101_13 s_101_14
        let s_101_15: Bits = Bits::new(s_101_13, s_101_14);
        // D s_101_16: cast reint s_101_15 -> u8
        let s_101_16: u8 = (s_101_15.value() as u8);
        // D s_101_17: cast zx s_101_16 -> bv
        let s_101_17: Bits = Bits::new(s_101_16 as u128, 2u16);
        // C s_101_18: const #0u : u8
        let s_101_18: u8 = 0;
        // C s_101_19: cast zx s_101_18 -> bv
        let s_101_19: Bits = Bits::new(s_101_18 as u128, 2u16);
        // D s_101_20: cmp-ne s_101_17 s_101_19
        let s_101_20: bool = ((s_101_17) != (s_101_19));
        // D s_101_21: write-var gs#109473 <= s_101_20
        fn_state.gs_109473 = s_101_20;
        // N s_101_22: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #432u : u32
        let s_102_0: u32 = 432;
        // D s_102_1: read-reg s_102_0:u8
        let s_102_1: u8 = {
            let value = state.read_register::<u8>(s_102_0 as isize);
            tracer.read_register(s_102_0 as isize, value);
            value
        };
        // D s_102_2: call ELUsingAArch32(s_102_1)
        let s_102_2: bool = ELUsingAArch32(state, tracer, s_102_1);
        // D s_102_3: not s_102_2
        let s_102_3: bool = !s_102_2;
        // D s_102_4: write-var gs#109472 <= s_102_3
        fn_state.gs_109472 = s_102_3;
        // N s_102_5: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_103_0: panic
        panic!("{:?}", ());
        // N s_103_1: return
        return;
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var __MDCR_EL3_TDA:u8
        let s_104_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_104_1: cast zx s_104_0 -> bv
        let s_104_1: Bits = Bits::new(s_104_0 as u128, 1u16);
        // C s_104_2: const #1u : u8
        let s_104_2: bool = true;
        // C s_104_3: cast zx s_104_2 -> bv
        let s_104_3: Bits = Bits::new(s_104_2 as u128, 1u16);
        // D s_104_4: cmp-eq s_104_1 s_104_3
        let s_104_4: bool = ((s_104_1) == (s_104_3));
        // D s_104_5: write-var gs#109471 <= s_104_4
        fn_state.gs_109471 = s_104_4;
        // N s_104_6: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #424u : u32
        let s_105_0: u32 = 424;
        // D s_105_1: read-reg s_105_0:u8
        let s_105_1: u8 = {
            let value = state.read_register::<u8>(s_105_0 as isize);
            tracer.read_register(s_105_0 as isize, value);
            value
        };
        // D s_105_2: call ELUsingAArch32(s_105_1)
        let s_105_2: bool = ELUsingAArch32(state, tracer, s_105_1);
        // D s_105_3: not s_105_2
        let s_105_3: bool = !s_105_2;
        // D s_105_4: write-var gs#109470 <= s_105_3
        fn_state.gs_109470 = s_105_3;
        // N s_105_5: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_106_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_106_1: call __IMPDEF_boolean(s_106_0)
        let s_106_1: bool = u__IMPDEF_boolean(state, tracer, s_106_0);
        // D s_106_2: write-var gs#109469 <= s_106_1
        fn_state.gs_109469 = s_106_1;
        // N s_106_3: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #() : ()
        let s_107_0: () = ();
        // S s_107_1: call EDSCR_read(s_107_0)
        let s_107_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_107_0);
        // S s_107_2: call _get_EDSCR_Type_SDD(s_107_1)
        let s_107_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_107_1);
        // S s_107_3: cast zx s_107_2 -> bv
        let s_107_3: Bits = Bits::new(s_107_2 as u128, 1u16);
        // C s_107_4: const #1u : u8
        let s_107_4: bool = true;
        // C s_107_5: cast zx s_107_4 -> bv
        let s_107_5: Bits = Bits::new(s_107_4 as u128, 1u16);
        // S s_107_6: cmp-eq s_107_3 s_107_5
        let s_107_6: bool = ((s_107_3) == (s_107_5));
        // D s_107_7: write-var gs#109468 <= s_107_6
        fn_state.gs_109468 = s_107_6;
        // N s_107_8: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #424u : u32
        let s_108_0: u32 = 424;
        // D s_108_1: read-reg s_108_0:u8
        let s_108_1: u8 = {
            let value = state.read_register::<u8>(s_108_0 as isize);
            tracer.read_register(s_108_0 as isize, value);
            value
        };
        // C s_108_2: const #2u : u8
        let s_108_2: u8 = 2;
        // D s_108_3: cmp-lt s_108_1 s_108_2
        let s_108_3: bool = ((s_108_1) < (s_108_2));
        // D s_108_4: write-var gs#109467 <= s_108_3
        fn_state.gs_109467 = s_108_3;
        // N s_108_5: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_109_0: panic
        panic!("{:?}", ());
        // N s_109_1: return
        return;
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_110_0: panic
        panic!("{:?}", ());
        // N s_110_1: return
        return;
    }
}
