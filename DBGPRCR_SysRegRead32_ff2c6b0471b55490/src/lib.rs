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
use Halted::*;
use u_get_HDCR_Type_TDE::*;
use HDCR_read::*;
use u_get_HDCR_Type_TDOSA::*;
use u__IMPDEF_boolean::*;
use u_get_MDCR_EL2_Type_TDE::*;
use AArch64_AArch32SystemAccessTrap::*;
use u__get_DBGPRCR::*;
use R_set::*;
use ELUsingAArch32::*;
use DBGPRCR_read::*;
use u_get_MDCR_EL3_Type_TDOSA::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_MDCR_EL2_Type_TDOSA::*;
use EL2Enabled::*;
use EDSCR_read::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn DBGPRCR_SysRegRead32_ff2c6b0471b55490<T: Tracer>(
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
        gs_109430: bool,
        gs_109427: bool,
        ga_173600: ProductType700c18a878c5601b,
        gs_109414: bool,
        gs_109421: bool,
        gs_109431: bool,
        gs_109423: bool,
        u__MDCR_EL3_TDOSA: bool,
        gs_109418: bool,
        gs_109419: bool,
        ga_173628: ProductType700c18a878c5601b,
        gs_109422: bool,
        gs_109412: bool,
        gs_109428: bool,
        gs_109420: bool,
        gs_109429: bool,
        gs_109416: bool,
        gs_109426: bool,
        gs_109413: bool,
        gs_109425: bool,
        u__PSTATE_EL: u8,
        gs_109417: bool,
        gs_109424: bool,
        ga_173624: ProductType700c18a878c5601b,
        gs_109415: bool,
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
        // D s_0_5: call _get_MDCR_EL3_Type_TDOSA(s_0_4)
        let s_0_5: bool = u_get_MDCR_EL3_Type_TDOSA(state, tracer, s_0_4);
        // D s_0_6: write-var __MDCR_EL3_TDOSA <= s_0_5
        fn_state.u__MDCR_EL3_TDOSA = s_0_5;
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
        // N s_0_13: branch s_0_12 b84 b1
        if s_0_12 {
            return block_84(state, tracer, fn_state);
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
        // S s_5_1: call DBGPRCR_read(s_5_0)
        let s_5_1: ProductType700c18a878c5601b = DBGPRCR_read(state, tracer, s_5_0);
        // S s_5_2: call __get_DBGPRCR(s_5_1)
        let s_5_2: ProductType700c18a878c5601b = u__get_DBGPRCR(state, tracer, s_5_1);
        // D s_5_3: write-var ga#173628 <= s_5_2
        fn_state.ga_173628 = s_5_2;
        // D s_5_4: read-var ga#173628.0:struct
        let s_5_4: u32 = fn_state.ga_173628._0;
        // D s_5_5: read-var t:i
        let s_5_5: i128 = fn_state.t;
        // D s_5_6: call R_set(s_5_5, s_5_4)
        let s_5_6: () = R_set(state, tracer, s_5_5, s_5_4);
        // N s_5_7: return
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
        // N s_6_2: branch s_6_1 b36 b7
        if s_6_1 {
            return block_36(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#109412 <= s_7_0
        fn_state.gs_109412 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#109412:u8
        let s_8_0: bool = fn_state.gs_109412;
        // N s_8_1: branch s_8_0 b35 b9
        if s_8_0 {
            return block_35(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#109413 <= s_9_0
        fn_state.gs_109413 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#109413:u8
        let s_10_0: bool = fn_state.gs_109413;
        // N s_10_1: branch s_10_0 b34 b11
        if s_10_0 {
            return block_34(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#109414 <= s_11_0
        fn_state.gs_109414 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#109414:u8
        let s_12_0: bool = fn_state.gs_109414;
        // N s_12_1: branch s_12_0 b33 b13
        if s_12_0 {
            return block_33(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#109415 <= s_13_0
        fn_state.gs_109415 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#109415:u8
        let s_14_0: bool = fn_state.gs_109415;
        // N s_14_1: branch s_14_0 b32 b15
        if s_14_0 {
            return block_32(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#109416 <= s_15_0
        fn_state.gs_109416 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#109416:u8
        let s_16_0: bool = fn_state.gs_109416;
        // N s_16_1: branch s_16_0 b31 b17
        if s_16_0 {
            return block_31(state, tracer, fn_state);
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
        // N s_17_4: branch s_17_3 b30 b18
        if s_17_3 {
            return block_30(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#109417 <= s_18_0
        fn_state.gs_109417 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#109417:u8
        let s_19_0: bool = fn_state.gs_109417;
        // N s_19_1: branch s_19_0 b29 b20
        if s_19_0 {
            return block_29(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#109418 <= s_20_0
        fn_state.gs_109418 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#109418:u8
        let s_21_0: bool = fn_state.gs_109418;
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
        // S s_22_1: call DBGPRCR_read(s_22_0)
        let s_22_1: ProductType700c18a878c5601b = DBGPRCR_read(state, tracer, s_22_0);
        // S s_22_2: call __get_DBGPRCR(s_22_1)
        let s_22_2: ProductType700c18a878c5601b = u__get_DBGPRCR(state, tracer, s_22_1);
        // D s_22_3: write-var ga#173624 <= s_22_2
        fn_state.ga_173624 = s_22_2;
        // D s_22_4: read-var ga#173624.0:struct
        let s_22_4: u32 = fn_state.ga_173624._0;
        // D s_22_5: read-var t:i
        let s_22_5: i128 = fn_state.t;
        // D s_22_6: call R_set(s_22_5, s_22_4)
        let s_22_6: () = R_set(state, tracer, s_22_5, s_22_4);
        // N s_22_7: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call Halted(s_23_0)
        let s_23_1: bool = Halted(state, tracer, s_23_0);
        // N s_23_2: branch s_23_1 b28 b24
        if s_23_1 {
            return block_28(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#109419 <= s_24_0
        fn_state.gs_109419 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#109419:u8
        let s_25_0: bool = fn_state.gs_109419;
        // N s_25_1: branch s_25_0 b27 b26
        if s_25_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #5u : u8
        let s_26_0: u8 = 5;
        // C s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 8u16);
        // C s_26_2: cast zx s_26_1 -> i
        let s_26_2: i128 = (s_26_1.value() as i128);
        // C s_26_3: cast reint s_26_2 -> i64
        let s_26_3: i64 = (s_26_2 as i64);
        // C s_26_4: cast zx s_26_3 -> i
        let s_26_4: i128 = (i128::try_from(s_26_3).unwrap());
        // C s_26_5: const #424u : u32
        let s_26_5: u32 = 424;
        // D s_26_6: read-reg s_26_5:u8
        let s_26_6: u8 = {
            let value = state.read_register::<u8>(s_26_5 as isize);
            tracer.read_register(s_26_5 as isize, value);
            value
        };
        // D s_26_7: call AArch64_AArch32SystemAccessTrap(s_26_6, s_26_4)
        let s_26_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_26_6, s_26_4);
        // N s_26_8: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: panic
        panic!("{:?}", ());
        // N s_27_1: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call EDSCR_read(s_28_0)
        let s_28_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_28_0);
        // S s_28_2: call _get_EDSCR_Type_SDD(s_28_1)
        let s_28_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_28_1);
        // S s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // C s_28_4: const #1u : u8
        let s_28_4: bool = true;
        // C s_28_5: cast zx s_28_4 -> bv
        let s_28_5: Bits = Bits::new(s_28_4 as u128, 1u16);
        // S s_28_6: cmp-eq s_28_3 s_28_5
        let s_28_6: bool = ((s_28_3) == (s_28_5));
        // D s_28_7: write-var gs#109419 <= s_28_6
        fn_state.gs_109419 = s_28_6;
        // N s_28_8: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var __MDCR_EL3_TDOSA:u8
        let s_29_0: bool = fn_state.u__MDCR_EL3_TDOSA;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #1u : u8
        let s_29_2: bool = true;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#109418 <= s_29_4
        fn_state.gs_109418 = s_29_4;
        // N s_29_6: jump b21
        return block_21(state, tracer, fn_state);
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
        // D s_30_2: call ELUsingAArch32(s_30_1)
        let s_30_2: bool = ELUsingAArch32(state, tracer, s_30_1);
        // D s_30_3: not s_30_2
        let s_30_3: bool = !s_30_2;
        // D s_30_4: write-var gs#109417 <= s_30_3
        fn_state.gs_109417 = s_30_3;
        // N s_30_5: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_31_0: panic
        panic!("{:?}", ());
        // N s_31_1: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var __MDCR_EL3_TDOSA:u8
        let s_32_0: bool = fn_state.u__MDCR_EL3_TDOSA;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // C s_32_2: const #1u : u8
        let s_32_2: bool = true;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // D s_32_5: write-var gs#109416 <= s_32_4
        fn_state.gs_109416 = s_32_4;
        // N s_32_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #424u : u32
        let s_33_0: u32 = 424;
        // D s_33_1: read-reg s_33_0:u8
        let s_33_1: u8 = {
            let value = state.read_register::<u8>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // D s_33_2: call ELUsingAArch32(s_33_1)
        let s_33_2: bool = ELUsingAArch32(state, tracer, s_33_1);
        // D s_33_3: not s_33_2
        let s_33_3: bool = !s_33_2;
        // D s_33_4: write-var gs#109415 <= s_33_3
        fn_state.gs_109415 = s_33_3;
        // N s_33_5: jump b14
        return block_14(state, tracer, fn_state);
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
        // D s_34_2: write-var gs#109414 <= s_34_1
        fn_state.gs_109414 = s_34_1;
        // N s_34_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #() : ()
        let s_35_0: () = ();
        // S s_35_1: call EDSCR_read(s_35_0)
        let s_35_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_35_0);
        // S s_35_2: call _get_EDSCR_Type_SDD(s_35_1)
        let s_35_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_35_1);
        // S s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 1u16);
        // C s_35_4: const #1u : u8
        let s_35_4: bool = true;
        // C s_35_5: cast zx s_35_4 -> bv
        let s_35_5: Bits = Bits::new(s_35_4 as u128, 1u16);
        // S s_35_6: cmp-eq s_35_3 s_35_5
        let s_35_6: bool = ((s_35_3) == (s_35_5));
        // D s_35_7: write-var gs#109413 <= s_35_6
        fn_state.gs_109413 = s_35_6;
        // N s_35_8: jump b10
        return block_10(state, tracer, fn_state);
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
        // D s_36_4: write-var gs#109412 <= s_36_3
        fn_state.gs_109412 = s_36_3;
        // N s_36_5: jump b8
        return block_8(state, tracer, fn_state);
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
        // N s_37_2: branch s_37_1 b83 b38
        if s_37_1 {
            return block_83(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#109420 <= s_38_0
        fn_state.gs_109420 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#109420:u8
        let s_39_0: bool = fn_state.gs_109420;
        // N s_39_1: branch s_39_0 b82 b40
        if s_39_0 {
            return block_82(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#109421 <= s_40_0
        fn_state.gs_109421 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#109421:u8
        let s_41_0: bool = fn_state.gs_109421;
        // N s_41_1: branch s_41_0 b81 b42
        if s_41_0 {
            return block_81(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#109422 <= s_42_0
        fn_state.gs_109422 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#109422:u8
        let s_43_0: bool = fn_state.gs_109422;
        // N s_43_1: branch s_43_0 b80 b44
        if s_43_0 {
            return block_80(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#109423 <= s_44_0
        fn_state.gs_109423 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#109423:u8
        let s_45_0: bool = fn_state.gs_109423;
        // N s_45_1: branch s_45_0 b79 b46
        if s_45_0 {
            return block_79(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#109424 <= s_46_0
        fn_state.gs_109424 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#109424:u8
        let s_47_0: bool = fn_state.gs_109424;
        // N s_47_1: branch s_47_0 b78 b48
        if s_47_0 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call EL2Enabled(s_48_0)
        let s_48_1: bool = EL2Enabled(state, tracer, s_48_0);
        // N s_48_2: branch s_48_1 b77 b49
        if s_48_1 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var gs#109425 <= s_49_0
        fn_state.gs_109425 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#109425:u8
        let s_50_0: bool = fn_state.gs_109425;
        // N s_50_1: branch s_50_0 b76 b51
        if s_50_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #0u : u8
        let s_51_0: bool = false;
        // D s_51_1: write-var gs#109426 <= s_51_0
        fn_state.gs_109426 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#109426:u8
        let s_52_0: bool = fn_state.gs_109426;
        // N s_52_1: branch s_52_0 b75 b53
        if s_52_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #() : ()
        let s_53_0: () = ();
        // S s_53_1: call EL2Enabled(s_53_0)
        let s_53_1: bool = EL2Enabled(state, tracer, s_53_0);
        // N s_53_2: branch s_53_1 b74 b54
        if s_53_1 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #0u : u8
        let s_54_0: bool = false;
        // D s_54_1: write-var gs#109427 <= s_54_0
        fn_state.gs_109427 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#109427:u8
        let s_55_0: bool = fn_state.gs_109427;
        // N s_55_1: branch s_55_0 b73 b56
        if s_55_0 {
            return block_73(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#109428 <= s_56_0
        fn_state.gs_109428 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#109428:u8
        let s_57_0: bool = fn_state.gs_109428;
        // N s_57_1: branch s_57_0 b72 b58
        if s_57_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #424u : u32
        let s_58_0: u32 = 424;
        // D s_58_1: read-reg s_58_0:u8
        let s_58_1: u8 = {
            let value = state.read_register::<u8>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // C s_58_2: const #2u : u8
        let s_58_2: u8 = 2;
        // D s_58_3: cmp-lt s_58_1 s_58_2
        let s_58_3: bool = ((s_58_1) < (s_58_2));
        // N s_58_4: branch s_58_3 b71 b59
        if s_58_3 {
            return block_71(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#109429 <= s_59_0
        fn_state.gs_109429 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#109429:u8
        let s_60_0: bool = fn_state.gs_109429;
        // N s_60_1: branch s_60_0 b70 b61
        if s_60_0 {
            return block_70(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#109430 <= s_61_0
        fn_state.gs_109430 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#109430:u8
        let s_62_0: bool = fn_state.gs_109430;
        // N s_62_1: branch s_62_0 b64 b63
        if s_62_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #() : ()
        let s_63_0: () = ();
        // S s_63_1: call DBGPRCR_read(s_63_0)
        let s_63_1: ProductType700c18a878c5601b = DBGPRCR_read(state, tracer, s_63_0);
        // S s_63_2: call __get_DBGPRCR(s_63_1)
        let s_63_2: ProductType700c18a878c5601b = u__get_DBGPRCR(state, tracer, s_63_1);
        // D s_63_3: write-var ga#173600 <= s_63_2
        fn_state.ga_173600 = s_63_2;
        // D s_63_4: read-var ga#173600.0:struct
        let s_63_4: u32 = fn_state.ga_173600._0;
        // D s_63_5: read-var t:i
        let s_63_5: i128 = fn_state.t;
        // D s_63_6: call R_set(s_63_5, s_63_4)
        let s_63_6: () = R_set(state, tracer, s_63_5, s_63_4);
        // N s_63_7: return
        return;
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
        // N s_64_2: branch s_64_1 b69 b65
        if s_64_1 {
            return block_69(state, tracer, fn_state);
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
        // D s_65_1: write-var gs#109431 <= s_65_0
        fn_state.gs_109431 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#109431:u8
        let s_66_0: bool = fn_state.gs_109431;
        // N s_66_1: branch s_66_0 b68 b67
        if s_66_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #5u : u8
        let s_67_0: u8 = 5;
        // C s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 8u16);
        // C s_67_2: cast zx s_67_1 -> i
        let s_67_2: i128 = (s_67_1.value() as i128);
        // C s_67_3: cast reint s_67_2 -> i64
        let s_67_3: i64 = (s_67_2 as i64);
        // C s_67_4: cast zx s_67_3 -> i
        let s_67_4: i128 = (i128::try_from(s_67_3).unwrap());
        // C s_67_5: const #424u : u32
        let s_67_5: u32 = 424;
        // D s_67_6: read-reg s_67_5:u8
        let s_67_6: u8 = {
            let value = state.read_register::<u8>(s_67_5 as isize);
            tracer.read_register(s_67_5 as isize, value);
            value
        };
        // D s_67_7: call AArch64_AArch32SystemAccessTrap(s_67_6, s_67_4)
        let s_67_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_67_6, s_67_4);
        // N s_67_8: return
        return;
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_68_0: panic
        panic!("{:?}", ());
        // N s_68_1: return
        return;
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #() : ()
        let s_69_0: () = ();
        // S s_69_1: call EDSCR_read(s_69_0)
        let s_69_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_69_0);
        // S s_69_2: call _get_EDSCR_Type_SDD(s_69_1)
        let s_69_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_69_1);
        // S s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // C s_69_4: const #1u : u8
        let s_69_4: bool = true;
        // C s_69_5: cast zx s_69_4 -> bv
        let s_69_5: Bits = Bits::new(s_69_4 as u128, 1u16);
        // S s_69_6: cmp-eq s_69_3 s_69_5
        let s_69_6: bool = ((s_69_3) == (s_69_5));
        // D s_69_7: write-var gs#109431 <= s_69_6
        fn_state.gs_109431 = s_69_6;
        // N s_69_8: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var __MDCR_EL3_TDOSA:u8
        let s_70_0: bool = fn_state.u__MDCR_EL3_TDOSA;
        // D s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 1u16);
        // C s_70_2: const #1u : u8
        let s_70_2: bool = true;
        // C s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 1u16);
        // D s_70_4: cmp-eq s_70_1 s_70_3
        let s_70_4: bool = ((s_70_1) == (s_70_3));
        // D s_70_5: write-var gs#109430 <= s_70_4
        fn_state.gs_109430 = s_70_4;
        // N s_70_6: jump b62
        return block_62(state, tracer, fn_state);
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
        // D s_71_2: call ELUsingAArch32(s_71_1)
        let s_71_2: bool = ELUsingAArch32(state, tracer, s_71_1);
        // D s_71_3: not s_71_2
        let s_71_3: bool = !s_71_2;
        // D s_71_4: write-var gs#109429 <= s_71_3
        fn_state.gs_109429 = s_71_3;
        // N s_71_5: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #5u : u8
        let s_72_0: u8 = 5;
        // C s_72_1: cast zx s_72_0 -> bv
        let s_72_1: Bits = Bits::new(s_72_0 as u128, 8u16);
        // C s_72_2: cast zx s_72_1 -> i
        let s_72_2: i128 = (s_72_1.value() as i128);
        // C s_72_3: cast reint s_72_2 -> i64
        let s_72_3: i64 = (s_72_2 as i64);
        // C s_72_4: cast zx s_72_3 -> i
        let s_72_4: i128 = (i128::try_from(s_72_3).unwrap());
        // S s_72_5: call AArch32_TakeHypTrapException(s_72_4)
        let s_72_5: () = AArch32_TakeHypTrapException(state, tracer, s_72_4);
        // N s_72_6: return
        return;
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #() : ()
        let s_73_0: () = ();
        // S s_73_1: call HDCR_read(s_73_0)
        let s_73_1: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_73_0);
        // S s_73_2: call _get_HDCR_Type_TDE(s_73_1)
        let s_73_2: bool = u_get_HDCR_Type_TDE(state, tracer, s_73_1);
        // C s_73_3: const #() : ()
        let s_73_3: () = ();
        // S s_73_4: call HDCR_read(s_73_3)
        let s_73_4: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_73_3);
        // S s_73_5: call _get_HDCR_Type_TDOSA(s_73_4)
        let s_73_5: bool = u_get_HDCR_Type_TDOSA(state, tracer, s_73_4);
        // S s_73_6: cast zx s_73_2 -> bv
        let s_73_6: Bits = Bits::new(s_73_2 as u128, 1u16);
        // S s_73_7: cast zx s_73_5 -> bv
        let s_73_7: Bits = Bits::new(s_73_5 as u128, 1u16);
        // S s_73_8: cast reint s_73_6 -> u128
        let s_73_8: u128 = (s_73_6.value() as u128);
        // D s_73_9: size-of s_73_6
        let s_73_9: u16 = s_73_6.length();
        // S s_73_10: cast reint s_73_7 -> u128
        let s_73_10: u128 = (s_73_7.value() as u128);
        // D s_73_11: size-of s_73_7
        let s_73_11: u16 = s_73_7.length();
        // D s_73_12: lsl s_73_8 s_73_11
        let s_73_12: u128 = s_73_8 << s_73_11;
        // D s_73_13: or s_73_12 s_73_10
        let s_73_13: u128 = ((s_73_12) | (s_73_10));
        // D s_73_14: add s_73_9 s_73_11
        let s_73_14: u16 = (s_73_9 + s_73_11);
        // D s_73_15: create-bits s_73_13 s_73_14
        let s_73_15: Bits = Bits::new(s_73_13, s_73_14);
        // D s_73_16: cast reint s_73_15 -> u8
        let s_73_16: u8 = (s_73_15.value() as u8);
        // D s_73_17: cast zx s_73_16 -> bv
        let s_73_17: Bits = Bits::new(s_73_16 as u128, 2u16);
        // C s_73_18: const #0u : u8
        let s_73_18: u8 = 0;
        // C s_73_19: cast zx s_73_18 -> bv
        let s_73_19: Bits = Bits::new(s_73_18 as u128, 2u16);
        // D s_73_20: cmp-ne s_73_17 s_73_19
        let s_73_20: bool = ((s_73_17) != (s_73_19));
        // D s_73_21: write-var gs#109428 <= s_73_20
        fn_state.gs_109428 = s_73_20;
        // N s_73_22: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #432u : u32
        let s_74_0: u32 = 432;
        // D s_74_1: read-reg s_74_0:u8
        let s_74_1: u8 = {
            let value = state.read_register::<u8>(s_74_0 as isize);
            tracer.read_register(s_74_0 as isize, value);
            value
        };
        // D s_74_2: call ELUsingAArch32(s_74_1)
        let s_74_2: bool = ELUsingAArch32(state, tracer, s_74_1);
        // D s_74_3: write-var gs#109427 <= s_74_2
        fn_state.gs_109427 = s_74_2;
        // N s_74_4: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #5u : u8
        let s_75_0: u8 = 5;
        // C s_75_1: cast zx s_75_0 -> bv
        let s_75_1: Bits = Bits::new(s_75_0 as u128, 8u16);
        // C s_75_2: cast zx s_75_1 -> i
        let s_75_2: i128 = (s_75_1.value() as i128);
        // C s_75_3: cast reint s_75_2 -> i64
        let s_75_3: i64 = (s_75_2 as i64);
        // C s_75_4: cast zx s_75_3 -> i
        let s_75_4: i128 = (i128::try_from(s_75_3).unwrap());
        // C s_75_5: const #432u : u32
        let s_75_5: u32 = 432;
        // D s_75_6: read-reg s_75_5:u8
        let s_75_6: u8 = {
            let value = state.read_register::<u8>(s_75_5 as isize);
            tracer.read_register(s_75_5 as isize, value);
            value
        };
        // D s_75_7: call AArch64_AArch32SystemAccessTrap(s_75_6, s_75_4)
        let s_75_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_75_6, s_75_4);
        // N s_75_8: return
        return;
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #104880u : u32
        let s_76_0: u32 = 104880;
        // D s_76_1: read-reg s_76_0:struct
        let s_76_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_76_0 as isize);
            tracer.read_register(s_76_0 as isize, value);
            value
        };
        // D s_76_2: call _get_MDCR_EL2_Type_TDE(s_76_1)
        let s_76_2: bool = u_get_MDCR_EL2_Type_TDE(state, tracer, s_76_1);
        // C s_76_3: const #104880u : u32
        let s_76_3: u32 = 104880;
        // D s_76_4: read-reg s_76_3:struct
        let s_76_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_76_3 as isize);
            tracer.read_register(s_76_3 as isize, value);
            value
        };
        // D s_76_5: call _get_MDCR_EL2_Type_TDOSA(s_76_4)
        let s_76_5: bool = u_get_MDCR_EL2_Type_TDOSA(state, tracer, s_76_4);
        // D s_76_6: cast zx s_76_2 -> bv
        let s_76_6: Bits = Bits::new(s_76_2 as u128, 1u16);
        // D s_76_7: cast zx s_76_5 -> bv
        let s_76_7: Bits = Bits::new(s_76_5 as u128, 1u16);
        // D s_76_8: cast reint s_76_6 -> u128
        let s_76_8: u128 = (s_76_6.value() as u128);
        // D s_76_9: size-of s_76_6
        let s_76_9: u16 = s_76_6.length();
        // D s_76_10: cast reint s_76_7 -> u128
        let s_76_10: u128 = (s_76_7.value() as u128);
        // D s_76_11: size-of s_76_7
        let s_76_11: u16 = s_76_7.length();
        // D s_76_12: lsl s_76_8 s_76_11
        let s_76_12: u128 = s_76_8 << s_76_11;
        // D s_76_13: or s_76_12 s_76_10
        let s_76_13: u128 = ((s_76_12) | (s_76_10));
        // D s_76_14: add s_76_9 s_76_11
        let s_76_14: u16 = (s_76_9 + s_76_11);
        // D s_76_15: create-bits s_76_13 s_76_14
        let s_76_15: Bits = Bits::new(s_76_13, s_76_14);
        // D s_76_16: cast reint s_76_15 -> u8
        let s_76_16: u8 = (s_76_15.value() as u8);
        // D s_76_17: cast zx s_76_16 -> bv
        let s_76_17: Bits = Bits::new(s_76_16 as u128, 2u16);
        // C s_76_18: const #0u : u8
        let s_76_18: u8 = 0;
        // C s_76_19: cast zx s_76_18 -> bv
        let s_76_19: Bits = Bits::new(s_76_18 as u128, 2u16);
        // D s_76_20: cmp-ne s_76_17 s_76_19
        let s_76_20: bool = ((s_76_17) != (s_76_19));
        // D s_76_21: write-var gs#109426 <= s_76_20
        fn_state.gs_109426 = s_76_20;
        // N s_76_22: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #432u : u32
        let s_77_0: u32 = 432;
        // D s_77_1: read-reg s_77_0:u8
        let s_77_1: u8 = {
            let value = state.read_register::<u8>(s_77_0 as isize);
            tracer.read_register(s_77_0 as isize, value);
            value
        };
        // D s_77_2: call ELUsingAArch32(s_77_1)
        let s_77_2: bool = ELUsingAArch32(state, tracer, s_77_1);
        // D s_77_3: not s_77_2
        let s_77_3: bool = !s_77_2;
        // D s_77_4: write-var gs#109425 <= s_77_3
        fn_state.gs_109425 = s_77_3;
        // N s_77_5: jump b50
        return block_50(state, tracer, fn_state);
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
        // D s_79_0: read-var __MDCR_EL3_TDOSA:u8
        let s_79_0: bool = fn_state.u__MDCR_EL3_TDOSA;
        // D s_79_1: cast zx s_79_0 -> bv
        let s_79_1: Bits = Bits::new(s_79_0 as u128, 1u16);
        // C s_79_2: const #1u : u8
        let s_79_2: bool = true;
        // C s_79_3: cast zx s_79_2 -> bv
        let s_79_3: Bits = Bits::new(s_79_2 as u128, 1u16);
        // D s_79_4: cmp-eq s_79_1 s_79_3
        let s_79_4: bool = ((s_79_1) == (s_79_3));
        // D s_79_5: write-var gs#109424 <= s_79_4
        fn_state.gs_109424 = s_79_4;
        // N s_79_6: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #424u : u32
        let s_80_0: u32 = 424;
        // D s_80_1: read-reg s_80_0:u8
        let s_80_1: u8 = {
            let value = state.read_register::<u8>(s_80_0 as isize);
            tracer.read_register(s_80_0 as isize, value);
            value
        };
        // D s_80_2: call ELUsingAArch32(s_80_1)
        let s_80_2: bool = ELUsingAArch32(state, tracer, s_80_1);
        // D s_80_3: not s_80_2
        let s_80_3: bool = !s_80_2;
        // D s_80_4: write-var gs#109423 <= s_80_3
        fn_state.gs_109423 = s_80_3;
        // N s_80_5: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_81_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_81_1: call __IMPDEF_boolean(s_81_0)
        let s_81_1: bool = u__IMPDEF_boolean(state, tracer, s_81_0);
        // D s_81_2: write-var gs#109422 <= s_81_1
        fn_state.gs_109422 = s_81_1;
        // N s_81_3: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #() : ()
        let s_82_0: () = ();
        // S s_82_1: call EDSCR_read(s_82_0)
        let s_82_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_82_0);
        // S s_82_2: call _get_EDSCR_Type_SDD(s_82_1)
        let s_82_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_82_1);
        // S s_82_3: cast zx s_82_2 -> bv
        let s_82_3: Bits = Bits::new(s_82_2 as u128, 1u16);
        // C s_82_4: const #1u : u8
        let s_82_4: bool = true;
        // C s_82_5: cast zx s_82_4 -> bv
        let s_82_5: Bits = Bits::new(s_82_4 as u128, 1u16);
        // S s_82_6: cmp-eq s_82_3 s_82_5
        let s_82_6: bool = ((s_82_3) == (s_82_5));
        // D s_82_7: write-var gs#109421 <= s_82_6
        fn_state.gs_109421 = s_82_6;
        // N s_82_8: jump b41
        return block_41(state, tracer, fn_state);
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
        // D s_83_4: write-var gs#109420 <= s_83_3
        fn_state.gs_109420 = s_83_3;
        // N s_83_5: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_84_0: panic
        panic!("{:?}", ());
        // N s_84_1: return
        return;
    }
}
