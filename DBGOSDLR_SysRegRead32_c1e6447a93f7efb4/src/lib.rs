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
use u_get_HDCR_Type_TDOSA::*;
use HDCR_read::*;
use Halted::*;
use u_get_HDCR_Type_TDE::*;
use IsFeatureImplemented::*;
use u__IMPDEF_boolean::*;
use u_get_MDCR_EL2_Type_TDE::*;
use AArch64_AArch32SystemAccessTrap::*;
use DBGOSDLR_read::*;
use R_set::*;
use ELUsingAArch32::*;
use u_get_MDCR_EL3_Type_TDOSA::*;
use u_get_MDCR_EL2_Type_TDOSA::*;
use u_get_EDSCR_Type_SDD::*;
use u__get_DBGOSDLR::*;
use EL2Enabled::*;
use EDSCR_read::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn DBGOSDLR_SysRegRead32_c1e6447a93f7efb4<T: Tracer>(
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
        gs_109361: bool,
        gs_109362: bool,
        gs_109339: bool,
        gs_109344: bool,
        gs_109340: bool,
        gs_109343: bool,
        gs_109347: bool,
        gs_109354: bool,
        gs_109363: bool,
        gs_109359: bool,
        gs_109365: bool,
        gs_109353: bool,
        gs_109338: bool,
        gs_109356: bool,
        gs_109346: bool,
        gs_109349: bool,
        ga_173373: ProductType700c18a878c5601b,
        gs_109358: bool,
        gs_109345: bool,
        ga_173407: ProductType700c18a878c5601b,
        gs_109352: bool,
        ga_173403: ProductType700c18a878c5601b,
        gs_109368: bool,
        gs_109367: bool,
        gs_109350: bool,
        gs_109364: bool,
        gs_109355: bool,
        gs_109337: bool,
        gs_109341: bool,
        u__PSTATE_EL: u8,
        gs_109366: bool,
        gs_109351: bool,
        gs_109357: bool,
        gs_109348: bool,
        gs_109342: bool,
        gs_109360: bool,
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
        // C s_0_3: const #104880u : u32
        let s_0_3: u32 = 104880;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_MDCR_EL2_Type_TDOSA(s_0_4)
        let s_0_5: bool = u_get_MDCR_EL2_Type_TDOSA(state, tracer, s_0_4);
        // C s_0_6: const #() : ()
        let s_0_6: () = ();
        // S s_0_7: call HDCR_read(s_0_6)
        let s_0_7: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_0_6);
        // S s_0_8: call _get_HDCR_Type_TDOSA(s_0_7)
        let s_0_8: bool = u_get_HDCR_Type_TDOSA(state, tracer, s_0_7);
        // D s_0_9: read-var __PSTATE_EL:u8
        let s_0_9: u8 = fn_state.u__PSTATE_EL;
        // D s_0_10: cast zx s_0_9 -> bv
        let s_0_10: Bits = Bits::new(s_0_9 as u128, 2u16);
        // C s_0_11: const #448u : u32
        let s_0_11: u32 = 448;
        // D s_0_12: read-reg s_0_11:u8
        let s_0_12: u8 = {
            let value = state.read_register::<u8>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 2u16);
        // D s_0_14: cmp-eq s_0_10 s_0_13
        let s_0_14: bool = ((s_0_10) == (s_0_13));
        // N s_0_15: branch s_0_14 b120 b1
        if s_0_14 {
            return block_120(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b49 b2
        if s_1_5 {
            return block_49(state, tracer, fn_state);
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
        // S s_5_1: call DBGOSDLR_read(s_5_0)
        let s_5_1: ProductType700c18a878c5601b = DBGOSDLR_read(state, tracer, s_5_0);
        // S s_5_2: call __get_DBGOSDLR(s_5_1)
        let s_5_2: ProductType700c18a878c5601b = u__get_DBGOSDLR(state, tracer, s_5_1);
        // D s_5_3: write-var ga#173407 <= s_5_2
        fn_state.ga_173407 = s_5_2;
        // D s_5_4: read-var ga#173407.0:struct
        let s_5_4: u32 = fn_state.ga_173407._0;
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
        // N s_6_2: branch s_6_1 b48 b7
        if s_6_1 {
            return block_48(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#109337 <= s_7_0
        fn_state.gs_109337 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#109337:u8
        let s_8_0: bool = fn_state.gs_109337;
        // N s_8_1: branch s_8_0 b47 b9
        if s_8_0 {
            return block_47(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#109338 <= s_9_0
        fn_state.gs_109338 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#109338:u8
        let s_10_0: bool = fn_state.gs_109338;
        // N s_10_1: branch s_10_0 b46 b11
        if s_10_0 {
            return block_46(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#109339 <= s_11_0
        fn_state.gs_109339 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#109339:u8
        let s_12_0: bool = fn_state.gs_109339;
        // N s_12_1: branch s_12_0 b45 b13
        if s_12_0 {
            return block_45(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#109340 <= s_13_0
        fn_state.gs_109340 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#109340:u8
        let s_14_0: bool = fn_state.gs_109340;
        // N s_14_1: branch s_14_0 b44 b15
        if s_14_0 {
            return block_44(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#109341 <= s_15_0
        fn_state.gs_109341 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#109341:u8
        let s_16_0: bool = fn_state.gs_109341;
        // N s_16_1: branch s_16_0 b40 b17
        if s_16_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#109343 <= s_17_0
        fn_state.gs_109343 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#109343:u8
        let s_18_0: bool = fn_state.gs_109343;
        // N s_18_1: branch s_18_0 b39 b19
        if s_18_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #424u : u32
        let s_19_0: u32 = 424;
        // D s_19_1: read-reg s_19_0:u8
        let s_19_1: u8 = {
            let value = state.read_register::<u8>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // C s_19_2: const #2u : u8
        let s_19_2: u8 = 2;
        // D s_19_3: cmp-lt s_19_1 s_19_2
        let s_19_3: bool = ((s_19_1) < (s_19_2));
        // N s_19_4: branch s_19_3 b38 b20
        if s_19_3 {
            return block_38(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#109344 <= s_20_0
        fn_state.gs_109344 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#109344:u8
        let s_21_0: bool = fn_state.gs_109344;
        // N s_21_1: branch s_21_0 b37 b22
        if s_21_0 {
            return block_37(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#109345 <= s_22_0
        fn_state.gs_109345 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#109345:u8
        let s_23_0: bool = fn_state.gs_109345;
        // N s_23_1: branch s_23_0 b33 b24
        if s_23_0 {
            return block_33(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#109347 <= s_24_0
        fn_state.gs_109347 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#109347:u8
        let s_25_0: bool = fn_state.gs_109347;
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
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call DBGOSDLR_read(s_26_0)
        let s_26_1: ProductType700c18a878c5601b = DBGOSDLR_read(state, tracer, s_26_0);
        // S s_26_2: call __get_DBGOSDLR(s_26_1)
        let s_26_2: ProductType700c18a878c5601b = u__get_DBGOSDLR(state, tracer, s_26_1);
        // D s_26_3: write-var ga#173403 <= s_26_2
        fn_state.ga_173403 = s_26_2;
        // D s_26_4: read-var ga#173403.0:struct
        let s_26_4: u32 = fn_state.ga_173403._0;
        // D s_26_5: read-var t:i
        let s_26_5: i128 = fn_state.t;
        // D s_26_6: call R_set(s_26_5, s_26_4)
        let s_26_6: () = R_set(state, tracer, s_26_5, s_26_4);
        // N s_26_7: return
        return;
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
        // N s_27_2: branch s_27_1 b32 b28
        if s_27_1 {
            return block_32(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#109348 <= s_28_0
        fn_state.gs_109348 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#109348:u8
        let s_29_0: bool = fn_state.gs_109348;
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
        // C s_30_0: const #5u : u8
        let s_30_0: u8 = 5;
        // C s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 8u16);
        // C s_30_2: cast zx s_30_1 -> i
        let s_30_2: i128 = (s_30_1.value() as i128);
        // C s_30_3: cast reint s_30_2 -> i64
        let s_30_3: i64 = (s_30_2 as i64);
        // C s_30_4: cast zx s_30_3 -> i
        let s_30_4: i128 = (i128::try_from(s_30_3).unwrap());
        // C s_30_5: const #424u : u32
        let s_30_5: u32 = 424;
        // D s_30_6: read-reg s_30_5:u8
        let s_30_6: u8 = {
            let value = state.read_register::<u8>(s_30_5 as isize);
            tracer.read_register(s_30_5 as isize, value);
            value
        };
        // D s_30_7: call AArch64_AArch32SystemAccessTrap(s_30_6, s_30_4)
        let s_30_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_30_6, s_30_4);
        // N s_30_8: return
        return;
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
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call EDSCR_read(s_32_0)
        let s_32_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_32_0);
        // S s_32_2: call _get_EDSCR_Type_SDD(s_32_1)
        let s_32_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_32_1);
        // S s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // C s_32_4: const #1u : u8
        let s_32_4: bool = true;
        // C s_32_5: cast zx s_32_4 -> bv
        let s_32_5: Bits = Bits::new(s_32_4 as u128, 1u16);
        // S s_32_6: cmp-eq s_32_3 s_32_5
        let s_32_6: bool = ((s_32_3) == (s_32_5));
        // D s_32_7: write-var gs#109348 <= s_32_6
        fn_state.gs_109348 = s_32_6;
        // N s_32_8: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #18u : u32
        let s_33_0: u32 = 18;
        // S s_33_1: call IsFeatureImplemented(s_33_0)
        let s_33_1: bool = IsFeatureImplemented(state, tracer, s_33_0);
        // N s_33_2: branch s_33_1 b36 b34
        if s_33_1 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #"Trapped by MDCR_EL3.TDOSA" : str
        let s_34_0: &'static str = "Trapped by MDCR_EL3.TDOSA";
        // S s_34_1: call __IMPDEF_boolean(s_34_0)
        let s_34_1: bool = u__IMPDEF_boolean(state, tracer, s_34_0);
        // D s_34_2: write-var gs#109346 <= s_34_1
        fn_state.gs_109346 = s_34_1;
        // N s_34_3: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#109346:u8
        let s_35_0: bool = fn_state.gs_109346;
        // D s_35_1: write-var gs#109347 <= s_35_0
        fn_state.gs_109347 = s_35_0;
        // N s_35_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #1u : u8
        let s_36_0: bool = true;
        // D s_36_1: write-var gs#109346 <= s_36_0
        fn_state.gs_109346 = s_36_0;
        // N s_36_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #22712u : u32
        let s_37_0: u32 = 22712;
        // D s_37_1: read-reg s_37_0:struct
        let s_37_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_37_0 as isize);
            tracer.read_register(s_37_0 as isize, value);
            value
        };
        // D s_37_2: call _get_MDCR_EL3_Type_TDOSA(s_37_1)
        let s_37_2: bool = u_get_MDCR_EL3_Type_TDOSA(state, tracer, s_37_1);
        // D s_37_3: cast zx s_37_2 -> bv
        let s_37_3: Bits = Bits::new(s_37_2 as u128, 1u16);
        // C s_37_4: const #1u : u8
        let s_37_4: bool = true;
        // C s_37_5: cast zx s_37_4 -> bv
        let s_37_5: Bits = Bits::new(s_37_4 as u128, 1u16);
        // D s_37_6: cmp-eq s_37_3 s_37_5
        let s_37_6: bool = ((s_37_3) == (s_37_5));
        // D s_37_7: write-var gs#109345 <= s_37_6
        fn_state.gs_109345 = s_37_6;
        // N s_37_8: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #424u : u32
        let s_38_0: u32 = 424;
        // D s_38_1: read-reg s_38_0:u8
        let s_38_1: u8 = {
            let value = state.read_register::<u8>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // D s_38_2: call ELUsingAArch32(s_38_1)
        let s_38_2: bool = ELUsingAArch32(state, tracer, s_38_1);
        // D s_38_3: not s_38_2
        let s_38_3: bool = !s_38_2;
        // D s_38_4: write-var gs#109344 <= s_38_3
        fn_state.gs_109344 = s_38_3;
        // N s_38_5: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_39_0: panic
        panic!("{:?}", ());
        // N s_39_1: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #18u : u32
        let s_40_0: u32 = 18;
        // S s_40_1: call IsFeatureImplemented(s_40_0)
        let s_40_1: bool = IsFeatureImplemented(state, tracer, s_40_0);
        // N s_40_2: branch s_40_1 b43 b41
        if s_40_1 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #"Trapped by MDCR_EL3.TDOSA" : str
        let s_41_0: &'static str = "Trapped by MDCR_EL3.TDOSA";
        // S s_41_1: call __IMPDEF_boolean(s_41_0)
        let s_41_1: bool = u__IMPDEF_boolean(state, tracer, s_41_0);
        // D s_41_2: write-var gs#109342 <= s_41_1
        fn_state.gs_109342 = s_41_1;
        // N s_41_3: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#109342:u8
        let s_42_0: bool = fn_state.gs_109342;
        // D s_42_1: write-var gs#109343 <= s_42_0
        fn_state.gs_109343 = s_42_0;
        // N s_42_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #1u : u8
        let s_43_0: bool = true;
        // D s_43_1: write-var gs#109342 <= s_43_0
        fn_state.gs_109342 = s_43_0;
        // N s_43_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #22712u : u32
        let s_44_0: u32 = 22712;
        // D s_44_1: read-reg s_44_0:struct
        let s_44_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_44_0 as isize);
            tracer.read_register(s_44_0 as isize, value);
            value
        };
        // D s_44_2: call _get_MDCR_EL3_Type_TDOSA(s_44_1)
        let s_44_2: bool = u_get_MDCR_EL3_Type_TDOSA(state, tracer, s_44_1);
        // D s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 1u16);
        // C s_44_4: const #1u : u8
        let s_44_4: bool = true;
        // C s_44_5: cast zx s_44_4 -> bv
        let s_44_5: Bits = Bits::new(s_44_4 as u128, 1u16);
        // D s_44_6: cmp-eq s_44_3 s_44_5
        let s_44_6: bool = ((s_44_3) == (s_44_5));
        // D s_44_7: write-var gs#109341 <= s_44_6
        fn_state.gs_109341 = s_44_6;
        // N s_44_8: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #424u : u32
        let s_45_0: u32 = 424;
        // D s_45_1: read-reg s_45_0:u8
        let s_45_1: u8 = {
            let value = state.read_register::<u8>(s_45_0 as isize);
            tracer.read_register(s_45_0 as isize, value);
            value
        };
        // D s_45_2: call ELUsingAArch32(s_45_1)
        let s_45_2: bool = ELUsingAArch32(state, tracer, s_45_1);
        // D s_45_3: not s_45_2
        let s_45_3: bool = !s_45_2;
        // D s_45_4: write-var gs#109340 <= s_45_3
        fn_state.gs_109340 = s_45_3;
        // N s_45_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_46_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_46_1: call __IMPDEF_boolean(s_46_0)
        let s_46_1: bool = u__IMPDEF_boolean(state, tracer, s_46_0);
        // D s_46_2: write-var gs#109339 <= s_46_1
        fn_state.gs_109339 = s_46_1;
        // N s_46_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // S s_47_1: call EDSCR_read(s_47_0)
        let s_47_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_47_0);
        // S s_47_2: call _get_EDSCR_Type_SDD(s_47_1)
        let s_47_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_47_1);
        // S s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 1u16);
        // C s_47_4: const #1u : u8
        let s_47_4: bool = true;
        // C s_47_5: cast zx s_47_4 -> bv
        let s_47_5: Bits = Bits::new(s_47_4 as u128, 1u16);
        // S s_47_6: cmp-eq s_47_3 s_47_5
        let s_47_6: bool = ((s_47_3) == (s_47_5));
        // D s_47_7: write-var gs#109338 <= s_47_6
        fn_state.gs_109338 = s_47_6;
        // N s_47_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #424u : u32
        let s_48_0: u32 = 424;
        // D s_48_1: read-reg s_48_0:u8
        let s_48_1: u8 = {
            let value = state.read_register::<u8>(s_48_0 as isize);
            tracer.read_register(s_48_0 as isize, value);
            value
        };
        // C s_48_2: const #2u : u8
        let s_48_2: u8 = 2;
        // D s_48_3: cmp-lt s_48_1 s_48_2
        let s_48_3: bool = ((s_48_1) < (s_48_2));
        // D s_48_4: write-var gs#109337 <= s_48_3
        fn_state.gs_109337 = s_48_3;
        // N s_48_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #() : ()
        let s_49_0: () = ();
        // S s_49_1: call Halted(s_49_0)
        let s_49_1: bool = Halted(state, tracer, s_49_0);
        // N s_49_2: branch s_49_1 b119 b50
        if s_49_1 {
            return block_119(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#109349 <= s_50_0
        fn_state.gs_109349 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#109349:u8
        let s_51_0: bool = fn_state.gs_109349;
        // N s_51_1: branch s_51_0 b118 b52
        if s_51_0 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #0u : u8
        let s_52_0: bool = false;
        // D s_52_1: write-var gs#109350 <= s_52_0
        fn_state.gs_109350 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#109350:u8
        let s_53_0: bool = fn_state.gs_109350;
        // N s_53_1: branch s_53_0 b117 b54
        if s_53_0 {
            return block_117(state, tracer, fn_state);
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
        // D s_54_1: write-var gs#109351 <= s_54_0
        fn_state.gs_109351 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#109351:u8
        let s_55_0: bool = fn_state.gs_109351;
        // N s_55_1: branch s_55_0 b116 b56
        if s_55_0 {
            return block_116(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#109352 <= s_56_0
        fn_state.gs_109352 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#109352:u8
        let s_57_0: bool = fn_state.gs_109352;
        // N s_57_1: branch s_57_0 b115 b58
        if s_57_0 {
            return block_115(state, tracer, fn_state);
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
        // D s_58_1: write-var gs#109353 <= s_58_0
        fn_state.gs_109353 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#109353:u8
        let s_59_0: bool = fn_state.gs_109353;
        // N s_59_1: branch s_59_0 b111 b60
        if s_59_0 {
            return block_111(state, tracer, fn_state);
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
        // D s_60_1: write-var gs#109355 <= s_60_0
        fn_state.gs_109355 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#109355:u8
        let s_61_0: bool = fn_state.gs_109355;
        // N s_61_1: branch s_61_0 b110 b62
        if s_61_0 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #() : ()
        let s_62_0: () = ();
        // S s_62_1: call EL2Enabled(s_62_0)
        let s_62_1: bool = EL2Enabled(state, tracer, s_62_0);
        // N s_62_2: branch s_62_1 b109 b63
        if s_62_1 {
            return block_109(state, tracer, fn_state);
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
        // D s_63_1: write-var gs#109356 <= s_63_0
        fn_state.gs_109356 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#109356:u8
        let s_64_0: bool = fn_state.gs_109356;
        // N s_64_1: branch s_64_0 b108 b65
        if s_64_0 {
            return block_108(state, tracer, fn_state);
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
        // D s_65_1: write-var gs#109357 <= s_65_0
        fn_state.gs_109357 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#109357:u8
        let s_66_0: bool = fn_state.gs_109357;
        // N s_66_1: branch s_66_0 b104 b67
        if s_66_0 {
            return block_104(state, tracer, fn_state);
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
        // D s_67_1: write-var gs#109359 <= s_67_0
        fn_state.gs_109359 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var gs#109359:u8
        let s_68_0: bool = fn_state.gs_109359;
        // N s_68_1: branch s_68_0 b103 b69
        if s_68_0 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #() : ()
        let s_69_0: () = ();
        // S s_69_1: call EL2Enabled(s_69_0)
        let s_69_1: bool = EL2Enabled(state, tracer, s_69_0);
        // N s_69_2: branch s_69_1 b102 b70
        if s_69_1 {
            return block_102(state, tracer, fn_state);
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
        // D s_70_1: write-var gs#109360 <= s_70_0
        fn_state.gs_109360 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#109360:u8
        let s_71_0: bool = fn_state.gs_109360;
        // N s_71_1: branch s_71_0 b101 b72
        if s_71_0 {
            return block_101(state, tracer, fn_state);
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
        // D s_72_1: write-var gs#109361 <= s_72_0
        fn_state.gs_109361 = s_72_0;
        // N s_72_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#109361:u8
        let s_73_0: bool = fn_state.gs_109361;
        // N s_73_1: branch s_73_0 b97 b74
        if s_73_0 {
            return block_97(state, tracer, fn_state);
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
        // D s_74_1: write-var gs#109363 <= s_74_0
        fn_state.gs_109363 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#109363:u8
        let s_75_0: bool = fn_state.gs_109363;
        // N s_75_1: branch s_75_0 b96 b76
        if s_75_0 {
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
        // C s_76_0: const #424u : u32
        let s_76_0: u32 = 424;
        // D s_76_1: read-reg s_76_0:u8
        let s_76_1: u8 = {
            let value = state.read_register::<u8>(s_76_0 as isize);
            tracer.read_register(s_76_0 as isize, value);
            value
        };
        // C s_76_2: const #2u : u8
        let s_76_2: u8 = 2;
        // D s_76_3: cmp-lt s_76_1 s_76_2
        let s_76_3: bool = ((s_76_1) < (s_76_2));
        // N s_76_4: branch s_76_3 b95 b77
        if s_76_3 {
            return block_95(state, tracer, fn_state);
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
        // D s_77_1: write-var gs#109364 <= s_77_0
        fn_state.gs_109364 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var gs#109364:u8
        let s_78_0: bool = fn_state.gs_109364;
        // N s_78_1: branch s_78_0 b94 b79
        if s_78_0 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #0u : u8
        let s_79_0: bool = false;
        // D s_79_1: write-var gs#109365 <= s_79_0
        fn_state.gs_109365 = s_79_0;
        // N s_79_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var gs#109365:u8
        let s_80_0: bool = fn_state.gs_109365;
        // N s_80_1: branch s_80_0 b90 b81
        if s_80_0 {
            return block_90(state, tracer, fn_state);
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
        // D s_81_1: write-var gs#109367 <= s_81_0
        fn_state.gs_109367 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#109367:u8
        let s_82_0: bool = fn_state.gs_109367;
        // N s_82_1: branch s_82_0 b84 b83
        if s_82_0 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #() : ()
        let s_83_0: () = ();
        // S s_83_1: call DBGOSDLR_read(s_83_0)
        let s_83_1: ProductType700c18a878c5601b = DBGOSDLR_read(state, tracer, s_83_0);
        // S s_83_2: call __get_DBGOSDLR(s_83_1)
        let s_83_2: ProductType700c18a878c5601b = u__get_DBGOSDLR(state, tracer, s_83_1);
        // D s_83_3: write-var ga#173373 <= s_83_2
        fn_state.ga_173373 = s_83_2;
        // D s_83_4: read-var ga#173373.0:struct
        let s_83_4: u32 = fn_state.ga_173373._0;
        // D s_83_5: read-var t:i
        let s_83_5: i128 = fn_state.t;
        // D s_83_6: call R_set(s_83_5, s_83_4)
        let s_83_6: () = R_set(state, tracer, s_83_5, s_83_4);
        // N s_83_7: return
        return;
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #() : ()
        let s_84_0: () = ();
        // S s_84_1: call Halted(s_84_0)
        let s_84_1: bool = Halted(state, tracer, s_84_0);
        // N s_84_2: branch s_84_1 b89 b85
        if s_84_1 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #0u : u8
        let s_85_0: bool = false;
        // D s_85_1: write-var gs#109368 <= s_85_0
        fn_state.gs_109368 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#109368:u8
        let s_86_0: bool = fn_state.gs_109368;
        // N s_86_1: branch s_86_0 b88 b87
        if s_86_0 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #5u : u8
        let s_87_0: u8 = 5;
        // C s_87_1: cast zx s_87_0 -> bv
        let s_87_1: Bits = Bits::new(s_87_0 as u128, 8u16);
        // C s_87_2: cast zx s_87_1 -> i
        let s_87_2: i128 = (s_87_1.value() as i128);
        // C s_87_3: cast reint s_87_2 -> i64
        let s_87_3: i64 = (s_87_2 as i64);
        // C s_87_4: cast zx s_87_3 -> i
        let s_87_4: i128 = (i128::try_from(s_87_3).unwrap());
        // C s_87_5: const #424u : u32
        let s_87_5: u32 = 424;
        // D s_87_6: read-reg s_87_5:u8
        let s_87_6: u8 = {
            let value = state.read_register::<u8>(s_87_5 as isize);
            tracer.read_register(s_87_5 as isize, value);
            value
        };
        // D s_87_7: call AArch64_AArch32SystemAccessTrap(s_87_6, s_87_4)
        let s_87_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_87_6, s_87_4);
        // N s_87_8: return
        return;
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_88_0: panic
        panic!("{:?}", ());
        // N s_88_1: return
        return;
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #() : ()
        let s_89_0: () = ();
        // S s_89_1: call EDSCR_read(s_89_0)
        let s_89_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_89_0);
        // S s_89_2: call _get_EDSCR_Type_SDD(s_89_1)
        let s_89_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_89_1);
        // S s_89_3: cast zx s_89_2 -> bv
        let s_89_3: Bits = Bits::new(s_89_2 as u128, 1u16);
        // C s_89_4: const #1u : u8
        let s_89_4: bool = true;
        // C s_89_5: cast zx s_89_4 -> bv
        let s_89_5: Bits = Bits::new(s_89_4 as u128, 1u16);
        // S s_89_6: cmp-eq s_89_3 s_89_5
        let s_89_6: bool = ((s_89_3) == (s_89_5));
        // D s_89_7: write-var gs#109368 <= s_89_6
        fn_state.gs_109368 = s_89_6;
        // N s_89_8: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #18u : u32
        let s_90_0: u32 = 18;
        // S s_90_1: call IsFeatureImplemented(s_90_0)
        let s_90_1: bool = IsFeatureImplemented(state, tracer, s_90_0);
        // N s_90_2: branch s_90_1 b93 b91
        if s_90_1 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #"Trapped by MDCR_EL3.TDOSA" : str
        let s_91_0: &'static str = "Trapped by MDCR_EL3.TDOSA";
        // S s_91_1: call __IMPDEF_boolean(s_91_0)
        let s_91_1: bool = u__IMPDEF_boolean(state, tracer, s_91_0);
        // D s_91_2: write-var gs#109366 <= s_91_1
        fn_state.gs_109366 = s_91_1;
        // N s_91_3: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#109366:u8
        let s_92_0: bool = fn_state.gs_109366;
        // D s_92_1: write-var gs#109367 <= s_92_0
        fn_state.gs_109367 = s_92_0;
        // N s_92_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #1u : u8
        let s_93_0: bool = true;
        // D s_93_1: write-var gs#109366 <= s_93_0
        fn_state.gs_109366 = s_93_0;
        // N s_93_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #22712u : u32
        let s_94_0: u32 = 22712;
        // D s_94_1: read-reg s_94_0:struct
        let s_94_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_94_0 as isize);
            tracer.read_register(s_94_0 as isize, value);
            value
        };
        // D s_94_2: call _get_MDCR_EL3_Type_TDOSA(s_94_1)
        let s_94_2: bool = u_get_MDCR_EL3_Type_TDOSA(state, tracer, s_94_1);
        // D s_94_3: cast zx s_94_2 -> bv
        let s_94_3: Bits = Bits::new(s_94_2 as u128, 1u16);
        // C s_94_4: const #1u : u8
        let s_94_4: bool = true;
        // C s_94_5: cast zx s_94_4 -> bv
        let s_94_5: Bits = Bits::new(s_94_4 as u128, 1u16);
        // D s_94_6: cmp-eq s_94_3 s_94_5
        let s_94_6: bool = ((s_94_3) == (s_94_5));
        // D s_94_7: write-var gs#109365 <= s_94_6
        fn_state.gs_109365 = s_94_6;
        // N s_94_8: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #424u : u32
        let s_95_0: u32 = 424;
        // D s_95_1: read-reg s_95_0:u8
        let s_95_1: u8 = {
            let value = state.read_register::<u8>(s_95_0 as isize);
            tracer.read_register(s_95_0 as isize, value);
            value
        };
        // D s_95_2: call ELUsingAArch32(s_95_1)
        let s_95_2: bool = ELUsingAArch32(state, tracer, s_95_1);
        // D s_95_3: not s_95_2
        let s_95_3: bool = !s_95_2;
        // D s_95_4: write-var gs#109364 <= s_95_3
        fn_state.gs_109364 = s_95_3;
        // N s_95_5: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #5u : u8
        let s_96_0: u8 = 5;
        // C s_96_1: cast zx s_96_0 -> bv
        let s_96_1: Bits = Bits::new(s_96_0 as u128, 8u16);
        // C s_96_2: cast zx s_96_1 -> i
        let s_96_2: i128 = (s_96_1.value() as i128);
        // C s_96_3: cast reint s_96_2 -> i64
        let s_96_3: i64 = (s_96_2 as i64);
        // C s_96_4: cast zx s_96_3 -> i
        let s_96_4: i128 = (i128::try_from(s_96_3).unwrap());
        // S s_96_5: call AArch32_TakeHypTrapException(s_96_4)
        let s_96_5: () = AArch32_TakeHypTrapException(state, tracer, s_96_4);
        // N s_96_6: return
        return;
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #18u : u32
        let s_97_0: u32 = 18;
        // S s_97_1: call IsFeatureImplemented(s_97_0)
        let s_97_1: bool = IsFeatureImplemented(state, tracer, s_97_0);
        // N s_97_2: branch s_97_1 b100 b98
        if s_97_1 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #"Trapped by HDCR.TDOSA" : str
        let s_98_0: &'static str = "Trapped by HDCR.TDOSA";
        // S s_98_1: call __IMPDEF_boolean(s_98_0)
        let s_98_1: bool = u__IMPDEF_boolean(state, tracer, s_98_0);
        // D s_98_2: write-var gs#109362 <= s_98_1
        fn_state.gs_109362 = s_98_1;
        // N s_98_3: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var gs#109362:u8
        let s_99_0: bool = fn_state.gs_109362;
        // D s_99_1: write-var gs#109363 <= s_99_0
        fn_state.gs_109363 = s_99_0;
        // N s_99_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #1u : u8
        let s_100_0: bool = true;
        // D s_100_1: write-var gs#109362 <= s_100_0
        fn_state.gs_109362 = s_100_0;
        // N s_100_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #() : ()
        let s_101_0: () = ();
        // S s_101_1: call HDCR_read(s_101_0)
        let s_101_1: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_101_0);
        // S s_101_2: call _get_HDCR_Type_TDE(s_101_1)
        let s_101_2: bool = u_get_HDCR_Type_TDE(state, tracer, s_101_1);
        // C s_101_3: const #() : ()
        let s_101_3: () = ();
        // S s_101_4: call HDCR_read(s_101_3)
        let s_101_4: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_101_3);
        // S s_101_5: call _get_HDCR_Type_TDOSA(s_101_4)
        let s_101_5: bool = u_get_HDCR_Type_TDOSA(state, tracer, s_101_4);
        // S s_101_6: cast zx s_101_2 -> bv
        let s_101_6: Bits = Bits::new(s_101_2 as u128, 1u16);
        // S s_101_7: cast zx s_101_5 -> bv
        let s_101_7: Bits = Bits::new(s_101_5 as u128, 1u16);
        // S s_101_8: cast reint s_101_6 -> u128
        let s_101_8: u128 = (s_101_6.value() as u128);
        // D s_101_9: size-of s_101_6
        let s_101_9: u16 = s_101_6.length();
        // S s_101_10: cast reint s_101_7 -> u128
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
        // D s_101_21: write-var gs#109361 <= s_101_20
        fn_state.gs_109361 = s_101_20;
        // N s_101_22: jump b73
        return block_73(state, tracer, fn_state);
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
        // D s_102_3: write-var gs#109360 <= s_102_2
        fn_state.gs_109360 = s_102_2;
        // N s_102_4: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #5u : u8
        let s_103_0: u8 = 5;
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
        // D s_103_7: call AArch64_AArch32SystemAccessTrap(s_103_6, s_103_4)
        let s_103_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_103_6,
            s_103_4,
        );
        // N s_103_8: return
        return;
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #18u : u32
        let s_104_0: u32 = 18;
        // S s_104_1: call IsFeatureImplemented(s_104_0)
        let s_104_1: bool = IsFeatureImplemented(state, tracer, s_104_0);
        // N s_104_2: branch s_104_1 b107 b105
        if s_104_1 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #"Trapped by MDCR_EL2.TDOSA" : str
        let s_105_0: &'static str = "Trapped by MDCR_EL2.TDOSA";
        // S s_105_1: call __IMPDEF_boolean(s_105_0)
        let s_105_1: bool = u__IMPDEF_boolean(state, tracer, s_105_0);
        // D s_105_2: write-var gs#109358 <= s_105_1
        fn_state.gs_109358 = s_105_1;
        // N s_105_3: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var gs#109358:u8
        let s_106_0: bool = fn_state.gs_109358;
        // D s_106_1: write-var gs#109359 <= s_106_0
        fn_state.gs_109359 = s_106_0;
        // N s_106_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #1u : u8
        let s_107_0: bool = true;
        // D s_107_1: write-var gs#109358 <= s_107_0
        fn_state.gs_109358 = s_107_0;
        // N s_107_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #104880u : u32
        let s_108_0: u32 = 104880;
        // D s_108_1: read-reg s_108_0:struct
        let s_108_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_108_0 as isize);
            tracer.read_register(s_108_0 as isize, value);
            value
        };
        // D s_108_2: call _get_MDCR_EL2_Type_TDE(s_108_1)
        let s_108_2: bool = u_get_MDCR_EL2_Type_TDE(state, tracer, s_108_1);
        // C s_108_3: const #104880u : u32
        let s_108_3: u32 = 104880;
        // D s_108_4: read-reg s_108_3:struct
        let s_108_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_108_3 as isize);
            tracer.read_register(s_108_3 as isize, value);
            value
        };
        // D s_108_5: call _get_MDCR_EL2_Type_TDOSA(s_108_4)
        let s_108_5: bool = u_get_MDCR_EL2_Type_TDOSA(state, tracer, s_108_4);
        // D s_108_6: cast zx s_108_2 -> bv
        let s_108_6: Bits = Bits::new(s_108_2 as u128, 1u16);
        // D s_108_7: cast zx s_108_5 -> bv
        let s_108_7: Bits = Bits::new(s_108_5 as u128, 1u16);
        // D s_108_8: cast reint s_108_6 -> u128
        let s_108_8: u128 = (s_108_6.value() as u128);
        // D s_108_9: size-of s_108_6
        let s_108_9: u16 = s_108_6.length();
        // D s_108_10: cast reint s_108_7 -> u128
        let s_108_10: u128 = (s_108_7.value() as u128);
        // D s_108_11: size-of s_108_7
        let s_108_11: u16 = s_108_7.length();
        // D s_108_12: lsl s_108_8 s_108_11
        let s_108_12: u128 = s_108_8 << s_108_11;
        // D s_108_13: or s_108_12 s_108_10
        let s_108_13: u128 = ((s_108_12) | (s_108_10));
        // D s_108_14: add s_108_9 s_108_11
        let s_108_14: u16 = (s_108_9 + s_108_11);
        // D s_108_15: create-bits s_108_13 s_108_14
        let s_108_15: Bits = Bits::new(s_108_13, s_108_14);
        // D s_108_16: cast reint s_108_15 -> u8
        let s_108_16: u8 = (s_108_15.value() as u8);
        // D s_108_17: cast zx s_108_16 -> bv
        let s_108_17: Bits = Bits::new(s_108_16 as u128, 2u16);
        // C s_108_18: const #0u : u8
        let s_108_18: u8 = 0;
        // C s_108_19: cast zx s_108_18 -> bv
        let s_108_19: Bits = Bits::new(s_108_18 as u128, 2u16);
        // D s_108_20: cmp-ne s_108_17 s_108_19
        let s_108_20: bool = ((s_108_17) != (s_108_19));
        // D s_108_21: write-var gs#109357 <= s_108_20
        fn_state.gs_109357 = s_108_20;
        // N s_108_22: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #432u : u32
        let s_109_0: u32 = 432;
        // D s_109_1: read-reg s_109_0:u8
        let s_109_1: u8 = {
            let value = state.read_register::<u8>(s_109_0 as isize);
            tracer.read_register(s_109_0 as isize, value);
            value
        };
        // D s_109_2: call ELUsingAArch32(s_109_1)
        let s_109_2: bool = ELUsingAArch32(state, tracer, s_109_1);
        // D s_109_3: not s_109_2
        let s_109_3: bool = !s_109_2;
        // D s_109_4: write-var gs#109356 <= s_109_3
        fn_state.gs_109356 = s_109_3;
        // N s_109_5: jump b64
        return block_64(state, tracer, fn_state);
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
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #18u : u32
        let s_111_0: u32 = 18;
        // S s_111_1: call IsFeatureImplemented(s_111_0)
        let s_111_1: bool = IsFeatureImplemented(state, tracer, s_111_0);
        // N s_111_2: branch s_111_1 b114 b112
        if s_111_1 {
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
        // C s_112_0: const #"Trapped by MDCR_EL3.TDOSA" : str
        let s_112_0: &'static str = "Trapped by MDCR_EL3.TDOSA";
        // S s_112_1: call __IMPDEF_boolean(s_112_0)
        let s_112_1: bool = u__IMPDEF_boolean(state, tracer, s_112_0);
        // D s_112_2: write-var gs#109354 <= s_112_1
        fn_state.gs_109354 = s_112_1;
        // N s_112_3: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#109354:u8
        let s_113_0: bool = fn_state.gs_109354;
        // D s_113_1: write-var gs#109355 <= s_113_0
        fn_state.gs_109355 = s_113_0;
        // N s_113_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #1u : u8
        let s_114_0: bool = true;
        // D s_114_1: write-var gs#109354 <= s_114_0
        fn_state.gs_109354 = s_114_0;
        // N s_114_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #22712u : u32
        let s_115_0: u32 = 22712;
        // D s_115_1: read-reg s_115_0:struct
        let s_115_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_115_0 as isize);
            tracer.read_register(s_115_0 as isize, value);
            value
        };
        // D s_115_2: call _get_MDCR_EL3_Type_TDOSA(s_115_1)
        let s_115_2: bool = u_get_MDCR_EL3_Type_TDOSA(state, tracer, s_115_1);
        // D s_115_3: cast zx s_115_2 -> bv
        let s_115_3: Bits = Bits::new(s_115_2 as u128, 1u16);
        // C s_115_4: const #1u : u8
        let s_115_4: bool = true;
        // C s_115_5: cast zx s_115_4 -> bv
        let s_115_5: Bits = Bits::new(s_115_4 as u128, 1u16);
        // D s_115_6: cmp-eq s_115_3 s_115_5
        let s_115_6: bool = ((s_115_3) == (s_115_5));
        // D s_115_7: write-var gs#109353 <= s_115_6
        fn_state.gs_109353 = s_115_6;
        // N s_115_8: jump b59
        return block_59(state, tracer, fn_state);
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
        // D s_116_2: call ELUsingAArch32(s_116_1)
        let s_116_2: bool = ELUsingAArch32(state, tracer, s_116_1);
        // D s_116_3: not s_116_2
        let s_116_3: bool = !s_116_2;
        // D s_116_4: write-var gs#109352 <= s_116_3
        fn_state.gs_109352 = s_116_3;
        // N s_116_5: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_117_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_117_1: call __IMPDEF_boolean(s_117_0)
        let s_117_1: bool = u__IMPDEF_boolean(state, tracer, s_117_0);
        // D s_117_2: write-var gs#109351 <= s_117_1
        fn_state.gs_109351 = s_117_1;
        // N s_117_3: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #() : ()
        let s_118_0: () = ();
        // S s_118_1: call EDSCR_read(s_118_0)
        let s_118_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_118_0);
        // S s_118_2: call _get_EDSCR_Type_SDD(s_118_1)
        let s_118_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_118_1);
        // S s_118_3: cast zx s_118_2 -> bv
        let s_118_3: Bits = Bits::new(s_118_2 as u128, 1u16);
        // C s_118_4: const #1u : u8
        let s_118_4: bool = true;
        // C s_118_5: cast zx s_118_4 -> bv
        let s_118_5: Bits = Bits::new(s_118_4 as u128, 1u16);
        // S s_118_6: cmp-eq s_118_3 s_118_5
        let s_118_6: bool = ((s_118_3) == (s_118_5));
        // D s_118_7: write-var gs#109350 <= s_118_6
        fn_state.gs_109350 = s_118_6;
        // N s_118_8: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #424u : u32
        let s_119_0: u32 = 424;
        // D s_119_1: read-reg s_119_0:u8
        let s_119_1: u8 = {
            let value = state.read_register::<u8>(s_119_0 as isize);
            tracer.read_register(s_119_0 as isize, value);
            value
        };
        // C s_119_2: const #2u : u8
        let s_119_2: u8 = 2;
        // D s_119_3: cmp-lt s_119_1 s_119_2
        let s_119_3: bool = ((s_119_1) < (s_119_2));
        // D s_119_4: write-var gs#109349 <= s_119_3
        fn_state.gs_109349 = s_119_3;
        // N s_119_5: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_120_0: panic
        panic!("{:?}", ());
        // N s_120_1: return
        return;
    }
}
