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
use HCR2_read::*;
use AArch32_TakeHypTrapException::*;
use u_get_SCR_Type_TERR::*;
use Halted::*;
use u_get_HSTR_EL2_Type_T5::*;
use u__IMPDEF_boolean::*;
use u_get_SCR_EL3_Type_TWERR::*;
use HSTR_read::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_HSTR_Type_T5::*;
use R_read::*;
use u_get_SCR_EL3_Type_TERR::*;
use ELUsingAArch32::*;
use u_get_HCR_EL2_Type_TERR::*;
use u_get_EDSCR_Type_SDD::*;
use ERXADDR2_write::*;
use u_get_HCR2_Type_TERR::*;
use EL2Enabled::*;
use AArch32_TakeMonitorTrapException::*;
use EDSCR_read::*;
use common::*;
pub fn ERXADDR2_SysRegWrite32_9f92470c72b2a640<T: Tracer>(
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
        gs_127643: bool,
        u__SCR_EL3_TWERR: bool,
        u__HSTR_EL2_T5: bool,
        gs_127676: bool,
        gs_127632: bool,
        gs_127644: bool,
        gs_127657: bool,
        gs_127664: bool,
        gs_127634: bool,
        gs_127659: bool,
        gs_127627: bool,
        gs_127647: bool,
        u__SCR_TERR: bool,
        gs_127650: bool,
        gs_127642: bool,
        gs_127678: bool,
        gs_127646: bool,
        gs_127654: bool,
        gs_127640: bool,
        gs_127653: bool,
        u__SCR_EL3_TERR: bool,
        gs_127625: bool,
        gs_127668: bool,
        gs_127648: bool,
        gs_127660: bool,
        gs_127662: bool,
        gs_127669: bool,
        gs_127681: bool,
        gs_127680: bool,
        gs_127671: bool,
        gs_127672: bool,
        gs_127639: bool,
        gs_127674: bool,
        gs_127630: bool,
        u__PSTATE_EL: u8,
        gs_127666: bool,
        gs_127663: bool,
        u__HCR_EL2_TERR: bool,
        gs_127651: bool,
        u__HCR2_TERR: bool,
        gs_127636: bool,
        gs_127629: bool,
        gs_127652: bool,
        gs_127661: bool,
        gs_127641: bool,
        gs_127677: bool,
        gs_127675: bool,
        gs_127656: bool,
        gs_127679: bool,
        gs_127628: bool,
        gs_127673: bool,
        gs_127635: bool,
        u__HSTR_T5: bool,
        gs_127667: bool,
        gs_127658: bool,
        gs_127649: bool,
        gs_127665: bool,
        gs_127682: bool,
        gs_127670: bool,
        u__PSTATE_M: u8,
        gs_127638: bool,
        gs_127626: bool,
        gs_127637: bool,
        gs_127645: bool,
        gs_127655: bool,
        gs_127631: bool,
        gs_127633: bool,
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
        // C s_0_3: const #90704u : u32
        let s_0_3: u32 = 90704;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_SCR_EL3_Type_TERR(s_0_4)
        let s_0_5: bool = u_get_SCR_EL3_Type_TERR(state, tracer, s_0_4);
        // D s_0_6: write-var __SCR_EL3_TERR <= s_0_5
        fn_state.u__SCR_EL3_TERR = s_0_5;
        // C s_0_7: const #90704u : u32
        let s_0_7: u32 = 90704;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_SCR_EL3_Type_TWERR(s_0_8)
        let s_0_9: bool = u_get_SCR_EL3_Type_TWERR(state, tracer, s_0_8);
        // D s_0_10: write-var __SCR_EL3_TWERR <= s_0_9
        fn_state.u__SCR_EL3_TWERR = s_0_9;
        // C s_0_11: const #20920u : u32
        let s_0_11: u32 = 20920;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_SCR_Type_TERR(s_0_12)
        let s_0_13: bool = u_get_SCR_Type_TERR(state, tracer, s_0_12);
        // D s_0_14: write-var __SCR_TERR <= s_0_13
        fn_state.u__SCR_TERR = s_0_13;
        // C s_0_15: const #104936u : u32
        let s_0_15: u32 = 104936;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_HSTR_EL2_Type_T5(s_0_16)
        let s_0_17: bool = u_get_HSTR_EL2_Type_T5(state, tracer, s_0_16);
        // D s_0_18: write-var __HSTR_EL2_T5 <= s_0_17
        fn_state.u__HSTR_EL2_T5 = s_0_17;
        // C s_0_19: const #() : ()
        let s_0_19: () = ();
        // S s_0_20: call HSTR_read(s_0_19)
        let s_0_20: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_19);
        // S s_0_21: call _get_HSTR_Type_T5(s_0_20)
        let s_0_21: bool = u_get_HSTR_Type_T5(state, tracer, s_0_20);
        // D s_0_22: write-var __HSTR_T5 <= s_0_21
        fn_state.u__HSTR_T5 = s_0_21;
        // C s_0_23: const #102552u : u32
        let s_0_23: u32 = 102552;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_HCR_EL2_Type_TERR(s_0_24)
        let s_0_25: bool = u_get_HCR_EL2_Type_TERR(state, tracer, s_0_24);
        // D s_0_26: write-var __HCR_EL2_TERR <= s_0_25
        fn_state.u__HCR_EL2_TERR = s_0_25;
        // C s_0_27: const #() : ()
        let s_0_27: () = ();
        // S s_0_28: call HCR2_read(s_0_27)
        let s_0_28: ProductType700c18a878c5601b = HCR2_read(state, tracer, s_0_27);
        // S s_0_29: call _get_HCR2_Type_TERR(s_0_28)
        let s_0_29: bool = u_get_HCR2_Type_TERR(state, tracer, s_0_28);
        // D s_0_30: write-var __HCR2_TERR <= s_0_29
        fn_state.u__HCR2_TERR = s_0_29;
        // C s_0_31: const #16983u : u32
        let s_0_31: u32 = 16983;
        // D s_0_32: read-reg s_0_31:u8
        let s_0_32: u8 = {
            let value = state.read_register::<u8>(s_0_31 as isize);
            tracer.read_register(s_0_31 as isize, value);
            value
        };
        // D s_0_33: write-var __PSTATE_M <= s_0_32
        fn_state.u__PSTATE_M = s_0_32;
        // D s_0_34: read-var __PSTATE_EL:u8
        let s_0_34: u8 = fn_state.u__PSTATE_EL;
        // D s_0_35: cast zx s_0_34 -> bv
        let s_0_35: Bits = Bits::new(s_0_34 as u128, 2u16);
        // C s_0_36: const #448u : u32
        let s_0_36: u32 = 448;
        // D s_0_37: read-reg s_0_36:u8
        let s_0_37: u8 = {
            let value = state.read_register::<u8>(s_0_36 as isize);
            tracer.read_register(s_0_36 as isize, value);
            value
        };
        // D s_0_38: cast zx s_0_37 -> bv
        let s_0_38: Bits = Bits::new(s_0_37 as u128, 2u16);
        // D s_0_39: cmp-eq s_0_35 s_0_38
        let s_0_39: bool = ((s_0_35) == (s_0_38));
        // N s_0_40: branch s_0_39 b228 b1
        if s_0_39 {
            return block_228(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b102 b2
        if s_1_5 {
            return block_102(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b11 b3
        if s_2_5 {
            return block_11(state, tracer, fn_state);
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
        // D s_5_0: read-var __PSTATE_M:u8
        let s_5_0: u8 = fn_state.u__PSTATE_M;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 5u16);
        // C s_5_2: const #384u : u32
        let s_5_2: u32 = 384;
        // D s_5_3: read-reg s_5_2:u8
        let s_5_3: u8 = {
            let value = state.read_register::<u8>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 5u16);
        // D s_5_5: cmp-ne s_5_1 s_5_4
        let s_5_5: bool = ((s_5_1) != (s_5_4));
        // N s_5_6: branch s_5_5 b10 b6
        if s_5_5 {
            return block_10(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#127625 <= s_6_0
        fn_state.gs_127625 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#127625:u8
        let s_7_0: bool = fn_state.gs_127625;
        // N s_7_1: branch s_7_0 b9 b8
        if s_7_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var t:i
        let s_8_0: i128 = fn_state.t;
        // D s_8_1: call R_read(s_8_0)
        let s_8_1: u32 = R_read(state, tracer, s_8_0);
        // D s_8_2: call ERXADDR2_write(s_8_1)
        let s_8_2: () = ERXADDR2_write(state, tracer, s_8_1);
        // N s_8_3: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call AArch32_TakeMonitorTrapException(s_9_0)
        let s_9_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_9_0);
        // N s_9_2: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var __SCR_TERR:u8
        let s_10_0: bool = fn_state.u__SCR_TERR;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // C s_10_2: const #1u : u8
        let s_10_2: bool = true;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // D s_10_5: write-var gs#127625 <= s_10_4
        fn_state.gs_127625 = s_10_4;
        // N s_10_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call Halted(s_11_0)
        let s_11_1: bool = Halted(state, tracer, s_11_0);
        // N s_11_2: branch s_11_1 b101 b12
        if s_11_1 {
            return block_101(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#127626 <= s_12_0
        fn_state.gs_127626 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#127626:u8
        let s_13_0: bool = fn_state.gs_127626;
        // N s_13_1: branch s_13_0 b100 b14
        if s_13_0 {
            return block_100(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#127627 <= s_14_0
        fn_state.gs_127627 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#127627:u8
        let s_15_0: bool = fn_state.gs_127627;
        // N s_15_1: branch s_15_0 b99 b16
        if s_15_0 {
            return block_99(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#127628 <= s_16_0
        fn_state.gs_127628 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#127628:u8
        let s_17_0: bool = fn_state.gs_127628;
        // N s_17_1: branch s_17_0 b98 b18
        if s_17_0 {
            return block_98(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#127629 <= s_18_0
        fn_state.gs_127629 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#127629:u8
        let s_19_0: bool = fn_state.gs_127629;
        // N s_19_1: branch s_19_0 b97 b20
        if s_19_0 {
            return block_97(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#127630 <= s_20_0
        fn_state.gs_127630 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#127630:u8
        let s_21_0: bool = fn_state.gs_127630;
        // N s_21_1: branch s_21_0 b96 b22
        if s_21_0 {
            return block_96(state, tracer, fn_state);
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
        // S s_22_1: call Halted(s_22_0)
        let s_22_1: bool = Halted(state, tracer, s_22_0);
        // N s_22_2: branch s_22_1 b95 b23
        if s_22_1 {
            return block_95(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#127631 <= s_23_0
        fn_state.gs_127631 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#127631:u8
        let s_24_0: bool = fn_state.gs_127631;
        // N s_24_1: branch s_24_0 b94 b25
        if s_24_0 {
            return block_94(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#127632 <= s_25_0
        fn_state.gs_127632 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#127632:u8
        let s_26_0: bool = fn_state.gs_127632;
        // N s_26_1: branch s_26_0 b93 b27
        if s_26_0 {
            return block_93(state, tracer, fn_state);
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
        // D s_27_1: write-var gs#127633 <= s_27_0
        fn_state.gs_127633 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#127633:u8
        let s_28_0: bool = fn_state.gs_127633;
        // N s_28_1: branch s_28_0 b92 b29
        if s_28_0 {
            return block_92(state, tracer, fn_state);
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
        // D s_29_1: write-var gs#127634 <= s_29_0
        fn_state.gs_127634 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#127634:u8
        let s_30_0: bool = fn_state.gs_127634;
        // N s_30_1: branch s_30_0 b91 b31
        if s_30_0 {
            return block_91(state, tracer, fn_state);
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
        // D s_31_1: write-var gs#127635 <= s_31_0
        fn_state.gs_127635 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#127635:u8
        let s_32_0: bool = fn_state.gs_127635;
        // N s_32_1: branch s_32_0 b90 b33
        if s_32_0 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
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
        // N s_33_2: branch s_33_1 b89 b34
        if s_33_1 {
            return block_89(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#127636 <= s_34_0
        fn_state.gs_127636 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#127636:u8
        let s_35_0: bool = fn_state.gs_127636;
        // N s_35_1: branch s_35_0 b88 b36
        if s_35_0 {
            return block_88(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#127637 <= s_36_0
        fn_state.gs_127637 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#127637:u8
        let s_37_0: bool = fn_state.gs_127637;
        // N s_37_1: branch s_37_0 b87 b38
        if s_37_0 {
            return block_87(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#127638 <= s_38_0
        fn_state.gs_127638 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#127638:u8
        let s_39_0: bool = fn_state.gs_127638;
        // N s_39_1: branch s_39_0 b86 b40
        if s_39_0 {
            return block_86(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#127639 <= s_40_0
        fn_state.gs_127639 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#127639:u8
        let s_41_0: bool = fn_state.gs_127639;
        // N s_41_1: branch s_41_0 b85 b42
        if s_41_0 {
            return block_85(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#127640 <= s_42_0
        fn_state.gs_127640 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#127640:u8
        let s_43_0: bool = fn_state.gs_127640;
        // N s_43_1: branch s_43_0 b84 b44
        if s_43_0 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #424u : u32
        let s_44_0: u32 = 424;
        // D s_44_1: read-reg s_44_0:u8
        let s_44_1: u8 = {
            let value = state.read_register::<u8>(s_44_0 as isize);
            tracer.read_register(s_44_0 as isize, value);
            value
        };
        // C s_44_2: const #2u : u8
        let s_44_2: u8 = 2;
        // D s_44_3: cmp-lt s_44_1 s_44_2
        let s_44_3: bool = ((s_44_1) < (s_44_2));
        // N s_44_4: branch s_44_3 b83 b45
        if s_44_3 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #0u : u8
        let s_45_0: bool = false;
        // D s_45_1: write-var gs#127641 <= s_45_0
        fn_state.gs_127641 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#127641:u8
        let s_46_0: bool = fn_state.gs_127641;
        // N s_46_1: branch s_46_0 b82 b47
        if s_46_0 {
            return block_82(state, tracer, fn_state);
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
        // D s_47_1: write-var gs#127642 <= s_47_0
        fn_state.gs_127642 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#127642:u8
        let s_48_0: bool = fn_state.gs_127642;
        // N s_48_1: branch s_48_0 b76 b49
        if s_48_0 {
            return block_76(state, tracer, fn_state);
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
        // N s_49_4: branch s_49_3 b75 b50
        if s_49_3 {
            return block_75(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#127643 <= s_50_0
        fn_state.gs_127643 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#127643:u8
        let s_51_0: bool = fn_state.gs_127643;
        // N s_51_1: branch s_51_0 b74 b52
        if s_51_0 {
            return block_74(state, tracer, fn_state);
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
        // D s_52_1: write-var gs#127644 <= s_52_0
        fn_state.gs_127644 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#127644:u8
        let s_53_0: bool = fn_state.gs_127644;
        // N s_53_1: branch s_53_0 b68 b54
        if s_53_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
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
        // N s_54_4: branch s_54_3 b67 b55
        if s_54_3 {
            return block_67(state, tracer, fn_state);
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
        // D s_55_1: write-var gs#127645 <= s_55_0
        fn_state.gs_127645 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#127645:u8
        let s_56_0: bool = fn_state.gs_127645;
        // N s_56_1: branch s_56_0 b66 b57
        if s_56_0 {
            return block_66(state, tracer, fn_state);
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
        // D s_57_1: write-var gs#127646 <= s_57_0
        fn_state.gs_127646 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#127646:u8
        let s_58_0: bool = fn_state.gs_127646;
        // N s_58_1: branch s_58_0 b60 b59
        if s_58_0 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var t:i
        let s_59_0: i128 = fn_state.t;
        // D s_59_1: call R_read(s_59_0)
        let s_59_1: u32 = R_read(state, tracer, s_59_0);
        // D s_59_2: call ERXADDR2_write(s_59_1)
        let s_59_2: () = ERXADDR2_write(state, tracer, s_59_1);
        // N s_59_3: return
        return;
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #() : ()
        let s_60_0: () = ();
        // S s_60_1: call Halted(s_60_0)
        let s_60_1: bool = Halted(state, tracer, s_60_0);
        // N s_60_2: branch s_60_1 b65 b61
        if s_60_1 {
            return block_65(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#127647 <= s_61_0
        fn_state.gs_127647 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#127647:u8
        let s_62_0: bool = fn_state.gs_127647;
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
        // S s_63_1: call AArch32_TakeMonitorTrapException(s_63_0)
        let s_63_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_63_0);
        // N s_63_2: return
        return;
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_64_0: panic
        panic!("{:?}", ());
        // N s_64_1: return
        return;
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #() : ()
        let s_65_0: () = ();
        // S s_65_1: call EDSCR_read(s_65_0)
        let s_65_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_65_0);
        // S s_65_2: call _get_EDSCR_Type_SDD(s_65_1)
        let s_65_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_65_1);
        // S s_65_3: cast zx s_65_2 -> bv
        let s_65_3: Bits = Bits::new(s_65_2 as u128, 1u16);
        // C s_65_4: const #1u : u8
        let s_65_4: bool = true;
        // C s_65_5: cast zx s_65_4 -> bv
        let s_65_5: Bits = Bits::new(s_65_4 as u128, 1u16);
        // S s_65_6: cmp-eq s_65_3 s_65_5
        let s_65_6: bool = ((s_65_3) == (s_65_5));
        // D s_65_7: write-var gs#127647 <= s_65_6
        fn_state.gs_127647 = s_65_6;
        // N s_65_8: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var __SCR_TERR:u8
        let s_66_0: bool = fn_state.u__SCR_TERR;
        // D s_66_1: cast zx s_66_0 -> bv
        let s_66_1: Bits = Bits::new(s_66_0 as u128, 1u16);
        // C s_66_2: const #1u : u8
        let s_66_2: bool = true;
        // C s_66_3: cast zx s_66_2 -> bv
        let s_66_3: Bits = Bits::new(s_66_2 as u128, 1u16);
        // D s_66_4: cmp-eq s_66_1 s_66_3
        let s_66_4: bool = ((s_66_1) == (s_66_3));
        // D s_66_5: write-var gs#127646 <= s_66_4
        fn_state.gs_127646 = s_66_4;
        // N s_66_6: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #424u : u32
        let s_67_0: u32 = 424;
        // D s_67_1: read-reg s_67_0:u8
        let s_67_1: u8 = {
            let value = state.read_register::<u8>(s_67_0 as isize);
            tracer.read_register(s_67_0 as isize, value);
            value
        };
        // D s_67_2: call ELUsingAArch32(s_67_1)
        let s_67_2: bool = ELUsingAArch32(state, tracer, s_67_1);
        // D s_67_3: write-var gs#127645 <= s_67_2
        fn_state.gs_127645 = s_67_2;
        // N s_67_4: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #() : ()
        let s_68_0: () = ();
        // S s_68_1: call Halted(s_68_0)
        let s_68_1: bool = Halted(state, tracer, s_68_0);
        // N s_68_2: branch s_68_1 b73 b69
        if s_68_1 {
            return block_73(state, tracer, fn_state);
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
        // D s_69_1: write-var gs#127648 <= s_69_0
        fn_state.gs_127648 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#127648:u8
        let s_70_0: bool = fn_state.gs_127648;
        // N s_70_1: branch s_70_0 b72 b71
        if s_70_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #3u : u8
        let s_71_0: u8 = 3;
        // C s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 8u16);
        // C s_71_2: cast zx s_71_1 -> i
        let s_71_2: i128 = (s_71_1.value() as i128);
        // C s_71_3: cast reint s_71_2 -> i64
        let s_71_3: i64 = (s_71_2 as i64);
        // C s_71_4: cast zx s_71_3 -> i
        let s_71_4: i128 = (i128::try_from(s_71_3).unwrap());
        // C s_71_5: const #424u : u32
        let s_71_5: u32 = 424;
        // D s_71_6: read-reg s_71_5:u8
        let s_71_6: u8 = {
            let value = state.read_register::<u8>(s_71_5 as isize);
            tracer.read_register(s_71_5 as isize, value);
            value
        };
        // D s_71_7: call AArch64_AArch32SystemAccessTrap(s_71_6, s_71_4)
        let s_71_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_71_6, s_71_4);
        // N s_71_8: return
        return;
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_72_0: panic
        panic!("{:?}", ());
        // N s_72_1: return
        return;
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #() : ()
        let s_73_0: () = ();
        // S s_73_1: call EDSCR_read(s_73_0)
        let s_73_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_73_0);
        // S s_73_2: call _get_EDSCR_Type_SDD(s_73_1)
        let s_73_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_73_1);
        // S s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 1u16);
        // C s_73_4: const #1u : u8
        let s_73_4: bool = true;
        // C s_73_5: cast zx s_73_4 -> bv
        let s_73_5: Bits = Bits::new(s_73_4 as u128, 1u16);
        // S s_73_6: cmp-eq s_73_3 s_73_5
        let s_73_6: bool = ((s_73_3) == (s_73_5));
        // D s_73_7: write-var gs#127648 <= s_73_6
        fn_state.gs_127648 = s_73_6;
        // N s_73_8: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var __SCR_EL3_TWERR:u8
        let s_74_0: bool = fn_state.u__SCR_EL3_TWERR;
        // D s_74_1: cast zx s_74_0 -> bv
        let s_74_1: Bits = Bits::new(s_74_0 as u128, 1u16);
        // C s_74_2: const #1u : u8
        let s_74_2: bool = true;
        // C s_74_3: cast zx s_74_2 -> bv
        let s_74_3: Bits = Bits::new(s_74_2 as u128, 1u16);
        // D s_74_4: cmp-eq s_74_1 s_74_3
        let s_74_4: bool = ((s_74_1) == (s_74_3));
        // D s_74_5: write-var gs#127644 <= s_74_4
        fn_state.gs_127644 = s_74_4;
        // N s_74_6: jump b53
        return block_53(state, tracer, fn_state);
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
        // D s_75_2: call ELUsingAArch32(s_75_1)
        let s_75_2: bool = ELUsingAArch32(state, tracer, s_75_1);
        // D s_75_3: not s_75_2
        let s_75_3: bool = !s_75_2;
        // D s_75_4: write-var gs#127643 <= s_75_3
        fn_state.gs_127643 = s_75_3;
        // N s_75_5: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #() : ()
        let s_76_0: () = ();
        // S s_76_1: call Halted(s_76_0)
        let s_76_1: bool = Halted(state, tracer, s_76_0);
        // N s_76_2: branch s_76_1 b81 b77
        if s_76_1 {
            return block_81(state, tracer, fn_state);
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
        // D s_77_1: write-var gs#127649 <= s_77_0
        fn_state.gs_127649 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var gs#127649:u8
        let s_78_0: bool = fn_state.gs_127649;
        // N s_78_1: branch s_78_0 b80 b79
        if s_78_0 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #3u : u8
        let s_79_0: u8 = 3;
        // C s_79_1: cast zx s_79_0 -> bv
        let s_79_1: Bits = Bits::new(s_79_0 as u128, 8u16);
        // C s_79_2: cast zx s_79_1 -> i
        let s_79_2: i128 = (s_79_1.value() as i128);
        // C s_79_3: cast reint s_79_2 -> i64
        let s_79_3: i64 = (s_79_2 as i64);
        // C s_79_4: cast zx s_79_3 -> i
        let s_79_4: i128 = (i128::try_from(s_79_3).unwrap());
        // C s_79_5: const #424u : u32
        let s_79_5: u32 = 424;
        // D s_79_6: read-reg s_79_5:u8
        let s_79_6: u8 = {
            let value = state.read_register::<u8>(s_79_5 as isize);
            tracer.read_register(s_79_5 as isize, value);
            value
        };
        // D s_79_7: call AArch64_AArch32SystemAccessTrap(s_79_6, s_79_4)
        let s_79_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_79_6, s_79_4);
        // N s_79_8: return
        return;
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_80_0: panic
        panic!("{:?}", ());
        // N s_80_1: return
        return;
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #() : ()
        let s_81_0: () = ();
        // S s_81_1: call EDSCR_read(s_81_0)
        let s_81_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_81_0);
        // S s_81_2: call _get_EDSCR_Type_SDD(s_81_1)
        let s_81_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_81_1);
        // S s_81_3: cast zx s_81_2 -> bv
        let s_81_3: Bits = Bits::new(s_81_2 as u128, 1u16);
        // C s_81_4: const #1u : u8
        let s_81_4: bool = true;
        // C s_81_5: cast zx s_81_4 -> bv
        let s_81_5: Bits = Bits::new(s_81_4 as u128, 1u16);
        // S s_81_6: cmp-eq s_81_3 s_81_5
        let s_81_6: bool = ((s_81_3) == (s_81_5));
        // D s_81_7: write-var gs#127649 <= s_81_6
        fn_state.gs_127649 = s_81_6;
        // N s_81_8: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var __SCR_EL3_TERR:u8
        let s_82_0: bool = fn_state.u__SCR_EL3_TERR;
        // D s_82_1: cast zx s_82_0 -> bv
        let s_82_1: Bits = Bits::new(s_82_0 as u128, 1u16);
        // C s_82_2: const #1u : u8
        let s_82_2: bool = true;
        // C s_82_3: cast zx s_82_2 -> bv
        let s_82_3: Bits = Bits::new(s_82_2 as u128, 1u16);
        // D s_82_4: cmp-eq s_82_1 s_82_3
        let s_82_4: bool = ((s_82_1) == (s_82_3));
        // D s_82_5: write-var gs#127642 <= s_82_4
        fn_state.gs_127642 = s_82_4;
        // N s_82_6: jump b48
        return block_48(state, tracer, fn_state);
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
        // D s_83_2: call ELUsingAArch32(s_83_1)
        let s_83_2: bool = ELUsingAArch32(state, tracer, s_83_1);
        // D s_83_3: not s_83_2
        let s_83_3: bool = !s_83_2;
        // D s_83_4: write-var gs#127641 <= s_83_3
        fn_state.gs_127641 = s_83_3;
        // N s_83_5: jump b46
        return block_46(state, tracer, fn_state);
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
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var __SCR_TERR:u8
        let s_85_0: bool = fn_state.u__SCR_TERR;
        // D s_85_1: cast zx s_85_0 -> bv
        let s_85_1: Bits = Bits::new(s_85_0 as u128, 1u16);
        // C s_85_2: const #1u : u8
        let s_85_2: bool = true;
        // C s_85_3: cast zx s_85_2 -> bv
        let s_85_3: Bits = Bits::new(s_85_2 as u128, 1u16);
        // D s_85_4: cmp-eq s_85_1 s_85_3
        let s_85_4: bool = ((s_85_1) == (s_85_3));
        // D s_85_5: write-var gs#127640 <= s_85_4
        fn_state.gs_127640 = s_85_4;
        // N s_85_6: jump b43
        return block_43(state, tracer, fn_state);
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
        // D s_86_2: call ELUsingAArch32(s_86_1)
        let s_86_2: bool = ELUsingAArch32(state, tracer, s_86_1);
        // D s_86_3: write-var gs#127639 <= s_86_2
        fn_state.gs_127639 = s_86_2;
        // N s_86_4: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_87_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_87_1: call __IMPDEF_boolean(s_87_0)
        let s_87_1: bool = u__IMPDEF_boolean(state, tracer, s_87_0);
        // D s_87_2: write-var gs#127638 <= s_87_1
        fn_state.gs_127638 = s_87_1;
        // N s_87_3: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #() : ()
        let s_88_0: () = ();
        // S s_88_1: call EDSCR_read(s_88_0)
        let s_88_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_88_0);
        // S s_88_2: call _get_EDSCR_Type_SDD(s_88_1)
        let s_88_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_88_1);
        // S s_88_3: cast zx s_88_2 -> bv
        let s_88_3: Bits = Bits::new(s_88_2 as u128, 1u16);
        // C s_88_4: const #1u : u8
        let s_88_4: bool = true;
        // C s_88_5: cast zx s_88_4 -> bv
        let s_88_5: Bits = Bits::new(s_88_4 as u128, 1u16);
        // S s_88_6: cmp-eq s_88_3 s_88_5
        let s_88_6: bool = ((s_88_3) == (s_88_5));
        // D s_88_7: write-var gs#127637 <= s_88_6
        fn_state.gs_127637 = s_88_6;
        // N s_88_8: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #424u : u32
        let s_89_0: u32 = 424;
        // D s_89_1: read-reg s_89_0:u8
        let s_89_1: u8 = {
            let value = state.read_register::<u8>(s_89_0 as isize);
            tracer.read_register(s_89_0 as isize, value);
            value
        };
        // C s_89_2: const #2u : u8
        let s_89_2: u8 = 2;
        // D s_89_3: cmp-lt s_89_1 s_89_2
        let s_89_3: bool = ((s_89_1) < (s_89_2));
        // D s_89_4: write-var gs#127636 <= s_89_3
        fn_state.gs_127636 = s_89_3;
        // N s_89_5: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_90_0: panic
        panic!("{:?}", ());
        // N s_90_1: return
        return;
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var __SCR_EL3_TWERR:u8
        let s_91_0: bool = fn_state.u__SCR_EL3_TWERR;
        // D s_91_1: cast zx s_91_0 -> bv
        let s_91_1: Bits = Bits::new(s_91_0 as u128, 1u16);
        // C s_91_2: const #1u : u8
        let s_91_2: bool = true;
        // C s_91_3: cast zx s_91_2 -> bv
        let s_91_3: Bits = Bits::new(s_91_2 as u128, 1u16);
        // D s_91_4: cmp-eq s_91_1 s_91_3
        let s_91_4: bool = ((s_91_1) == (s_91_3));
        // D s_91_5: write-var gs#127635 <= s_91_4
        fn_state.gs_127635 = s_91_4;
        // N s_91_6: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #424u : u32
        let s_92_0: u32 = 424;
        // D s_92_1: read-reg s_92_0:u8
        let s_92_1: u8 = {
            let value = state.read_register::<u8>(s_92_0 as isize);
            tracer.read_register(s_92_0 as isize, value);
            value
        };
        // D s_92_2: call ELUsingAArch32(s_92_1)
        let s_92_2: bool = ELUsingAArch32(state, tracer, s_92_1);
        // D s_92_3: not s_92_2
        let s_92_3: bool = !s_92_2;
        // D s_92_4: write-var gs#127634 <= s_92_3
        fn_state.gs_127634 = s_92_3;
        // N s_92_5: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_93_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_93_1: call __IMPDEF_boolean(s_93_0)
        let s_93_1: bool = u__IMPDEF_boolean(state, tracer, s_93_0);
        // D s_93_2: write-var gs#127633 <= s_93_1
        fn_state.gs_127633 = s_93_1;
        // N s_93_3: jump b28
        return block_28(state, tracer, fn_state);
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
        // D s_94_7: write-var gs#127632 <= s_94_6
        fn_state.gs_127632 = s_94_6;
        // N s_94_8: jump b26
        return block_26(state, tracer, fn_state);
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
        // C s_95_2: const #2u : u8
        let s_95_2: u8 = 2;
        // D s_95_3: cmp-lt s_95_1 s_95_2
        let s_95_3: bool = ((s_95_1) < (s_95_2));
        // D s_95_4: write-var gs#127631 <= s_95_3
        fn_state.gs_127631 = s_95_3;
        // N s_95_5: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_96_0: panic
        panic!("{:?}", ());
        // N s_96_1: return
        return;
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var __SCR_EL3_TERR:u8
        let s_97_0: bool = fn_state.u__SCR_EL3_TERR;
        // D s_97_1: cast zx s_97_0 -> bv
        let s_97_1: Bits = Bits::new(s_97_0 as u128, 1u16);
        // C s_97_2: const #1u : u8
        let s_97_2: bool = true;
        // C s_97_3: cast zx s_97_2 -> bv
        let s_97_3: Bits = Bits::new(s_97_2 as u128, 1u16);
        // D s_97_4: cmp-eq s_97_1 s_97_3
        let s_97_4: bool = ((s_97_1) == (s_97_3));
        // D s_97_5: write-var gs#127630 <= s_97_4
        fn_state.gs_127630 = s_97_4;
        // N s_97_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #424u : u32
        let s_98_0: u32 = 424;
        // D s_98_1: read-reg s_98_0:u8
        let s_98_1: u8 = {
            let value = state.read_register::<u8>(s_98_0 as isize);
            tracer.read_register(s_98_0 as isize, value);
            value
        };
        // D s_98_2: call ELUsingAArch32(s_98_1)
        let s_98_2: bool = ELUsingAArch32(state, tracer, s_98_1);
        // D s_98_3: not s_98_2
        let s_98_3: bool = !s_98_2;
        // D s_98_4: write-var gs#127629 <= s_98_3
        fn_state.gs_127629 = s_98_3;
        // N s_98_5: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_99_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_99_1: call __IMPDEF_boolean(s_99_0)
        let s_99_1: bool = u__IMPDEF_boolean(state, tracer, s_99_0);
        // D s_99_2: write-var gs#127628 <= s_99_1
        fn_state.gs_127628 = s_99_1;
        // N s_99_3: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #() : ()
        let s_100_0: () = ();
        // S s_100_1: call EDSCR_read(s_100_0)
        let s_100_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_100_0);
        // S s_100_2: call _get_EDSCR_Type_SDD(s_100_1)
        let s_100_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_100_1);
        // S s_100_3: cast zx s_100_2 -> bv
        let s_100_3: Bits = Bits::new(s_100_2 as u128, 1u16);
        // C s_100_4: const #1u : u8
        let s_100_4: bool = true;
        // C s_100_5: cast zx s_100_4 -> bv
        let s_100_5: Bits = Bits::new(s_100_4 as u128, 1u16);
        // S s_100_6: cmp-eq s_100_3 s_100_5
        let s_100_6: bool = ((s_100_3) == (s_100_5));
        // D s_100_7: write-var gs#127627 <= s_100_6
        fn_state.gs_127627 = s_100_6;
        // N s_100_8: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #424u : u32
        let s_101_0: u32 = 424;
        // D s_101_1: read-reg s_101_0:u8
        let s_101_1: u8 = {
            let value = state.read_register::<u8>(s_101_0 as isize);
            tracer.read_register(s_101_0 as isize, value);
            value
        };
        // C s_101_2: const #2u : u8
        let s_101_2: u8 = 2;
        // D s_101_3: cmp-lt s_101_1 s_101_2
        let s_101_3: bool = ((s_101_1) < (s_101_2));
        // D s_101_4: write-var gs#127626 <= s_101_3
        fn_state.gs_127626 = s_101_3;
        // N s_101_5: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #() : ()
        let s_102_0: () = ();
        // S s_102_1: call Halted(s_102_0)
        let s_102_1: bool = Halted(state, tracer, s_102_0);
        // N s_102_2: branch s_102_1 b227 b103
        if s_102_1 {
            return block_227(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #0u : u8
        let s_103_0: bool = false;
        // D s_103_1: write-var gs#127650 <= s_103_0
        fn_state.gs_127650 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var gs#127650:u8
        let s_104_0: bool = fn_state.gs_127650;
        // N s_104_1: branch s_104_0 b226 b105
        if s_104_0 {
            return block_226(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #0u : u8
        let s_105_0: bool = false;
        // D s_105_1: write-var gs#127651 <= s_105_0
        fn_state.gs_127651 = s_105_0;
        // N s_105_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var gs#127651:u8
        let s_106_0: bool = fn_state.gs_127651;
        // N s_106_1: branch s_106_0 b225 b107
        if s_106_0 {
            return block_225(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #0u : u8
        let s_107_0: bool = false;
        // D s_107_1: write-var gs#127652 <= s_107_0
        fn_state.gs_127652 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#127652:u8
        let s_108_0: bool = fn_state.gs_127652;
        // N s_108_1: branch s_108_0 b224 b109
        if s_108_0 {
            return block_224(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #0u : u8
        let s_109_0: bool = false;
        // D s_109_1: write-var gs#127653 <= s_109_0
        fn_state.gs_127653 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var gs#127653:u8
        let s_110_0: bool = fn_state.gs_127653;
        // N s_110_1: branch s_110_0 b223 b111
        if s_110_0 {
            return block_223(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #0u : u8
        let s_111_0: bool = false;
        // D s_111_1: write-var gs#127654 <= s_111_0
        fn_state.gs_127654 = s_111_0;
        // N s_111_2: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var gs#127654:u8
        let s_112_0: bool = fn_state.gs_127654;
        // N s_112_1: branch s_112_0 b222 b113
        if s_112_0 {
            return block_222(state, tracer, fn_state);
        } else {
            return block_113(state, tracer, fn_state);
        };
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #() : ()
        let s_113_0: () = ();
        // S s_113_1: call Halted(s_113_0)
        let s_113_1: bool = Halted(state, tracer, s_113_0);
        // N s_113_2: branch s_113_1 b221 b114
        if s_113_1 {
            return block_221(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #0u : u8
        let s_114_0: bool = false;
        // D s_114_1: write-var gs#127655 <= s_114_0
        fn_state.gs_127655 = s_114_0;
        // N s_114_2: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var gs#127655:u8
        let s_115_0: bool = fn_state.gs_127655;
        // N s_115_1: branch s_115_0 b220 b116
        if s_115_0 {
            return block_220(state, tracer, fn_state);
        } else {
            return block_116(state, tracer, fn_state);
        };
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #0u : u8
        let s_116_0: bool = false;
        // D s_116_1: write-var gs#127656 <= s_116_0
        fn_state.gs_127656 = s_116_0;
        // N s_116_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var gs#127656:u8
        let s_117_0: bool = fn_state.gs_127656;
        // N s_117_1: branch s_117_0 b219 b118
        if s_117_0 {
            return block_219(state, tracer, fn_state);
        } else {
            return block_118(state, tracer, fn_state);
        };
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #0u : u8
        let s_118_0: bool = false;
        // D s_118_1: write-var gs#127657 <= s_118_0
        fn_state.gs_127657 = s_118_0;
        // N s_118_2: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var gs#127657:u8
        let s_119_0: bool = fn_state.gs_127657;
        // N s_119_1: branch s_119_0 b218 b120
        if s_119_0 {
            return block_218(state, tracer, fn_state);
        } else {
            return block_120(state, tracer, fn_state);
        };
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #0u : u8
        let s_120_0: bool = false;
        // D s_120_1: write-var gs#127658 <= s_120_0
        fn_state.gs_127658 = s_120_0;
        // N s_120_2: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var gs#127658:u8
        let s_121_0: bool = fn_state.gs_127658;
        // N s_121_1: branch s_121_0 b217 b122
        if s_121_0 {
            return block_217(state, tracer, fn_state);
        } else {
            return block_122(state, tracer, fn_state);
        };
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #0u : u8
        let s_122_0: bool = false;
        // D s_122_1: write-var gs#127659 <= s_122_0
        fn_state.gs_127659 = s_122_0;
        // N s_122_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var gs#127659:u8
        let s_123_0: bool = fn_state.gs_127659;
        // N s_123_1: branch s_123_0 b216 b124
        if s_123_0 {
            return block_216(state, tracer, fn_state);
        } else {
            return block_124(state, tracer, fn_state);
        };
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #() : ()
        let s_124_0: () = ();
        // S s_124_1: call Halted(s_124_0)
        let s_124_1: bool = Halted(state, tracer, s_124_0);
        // N s_124_2: branch s_124_1 b215 b125
        if s_124_1 {
            return block_215(state, tracer, fn_state);
        } else {
            return block_125(state, tracer, fn_state);
        };
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #0u : u8
        let s_125_0: bool = false;
        // D s_125_1: write-var gs#127660 <= s_125_0
        fn_state.gs_127660 = s_125_0;
        // N s_125_2: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var gs#127660:u8
        let s_126_0: bool = fn_state.gs_127660;
        // N s_126_1: branch s_126_0 b214 b127
        if s_126_0 {
            return block_214(state, tracer, fn_state);
        } else {
            return block_127(state, tracer, fn_state);
        };
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #0u : u8
        let s_127_0: bool = false;
        // D s_127_1: write-var gs#127661 <= s_127_0
        fn_state.gs_127661 = s_127_0;
        // N s_127_2: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_128_0: read-var gs#127661:u8
        let s_128_0: bool = fn_state.gs_127661;
        // N s_128_1: branch s_128_0 b213 b129
        if s_128_0 {
            return block_213(state, tracer, fn_state);
        } else {
            return block_129(state, tracer, fn_state);
        };
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #0u : u8
        let s_129_0: bool = false;
        // D s_129_1: write-var gs#127662 <= s_129_0
        fn_state.gs_127662 = s_129_0;
        // N s_129_2: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_130_0: read-var gs#127662:u8
        let s_130_0: bool = fn_state.gs_127662;
        // N s_130_1: branch s_130_0 b212 b131
        if s_130_0 {
            return block_212(state, tracer, fn_state);
        } else {
            return block_131(state, tracer, fn_state);
        };
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #0u : u8
        let s_131_0: bool = false;
        // D s_131_1: write-var gs#127663 <= s_131_0
        fn_state.gs_127663 = s_131_0;
        // N s_131_2: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_132_0: read-var gs#127663:u8
        let s_132_0: bool = fn_state.gs_127663;
        // N s_132_1: branch s_132_0 b211 b133
        if s_132_0 {
            return block_211(state, tracer, fn_state);
        } else {
            return block_133(state, tracer, fn_state);
        };
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #0u : u8
        let s_133_0: bool = false;
        // D s_133_1: write-var gs#127664 <= s_133_0
        fn_state.gs_127664 = s_133_0;
        // N s_133_2: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var gs#127664:u8
        let s_134_0: bool = fn_state.gs_127664;
        // N s_134_1: branch s_134_0 b210 b135
        if s_134_0 {
            return block_210(state, tracer, fn_state);
        } else {
            return block_135(state, tracer, fn_state);
        };
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #() : ()
        let s_135_0: () = ();
        // S s_135_1: call EL2Enabled(s_135_0)
        let s_135_1: bool = EL2Enabled(state, tracer, s_135_0);
        // N s_135_2: branch s_135_1 b209 b136
        if s_135_1 {
            return block_209(state, tracer, fn_state);
        } else {
            return block_136(state, tracer, fn_state);
        };
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #0u : u8
        let s_136_0: bool = false;
        // D s_136_1: write-var gs#127665 <= s_136_0
        fn_state.gs_127665 = s_136_0;
        // N s_136_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var gs#127665:u8
        let s_137_0: bool = fn_state.gs_127665;
        // N s_137_1: branch s_137_0 b208 b138
        if s_137_0 {
            return block_208(state, tracer, fn_state);
        } else {
            return block_138(state, tracer, fn_state);
        };
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_138_0: const #0u : u8
        let s_138_0: bool = false;
        // D s_138_1: write-var gs#127666 <= s_138_0
        fn_state.gs_127666 = s_138_0;
        // N s_138_2: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var gs#127666:u8
        let s_139_0: bool = fn_state.gs_127666;
        // N s_139_1: branch s_139_0 b207 b140
        if s_139_0 {
            return block_207(state, tracer, fn_state);
        } else {
            return block_140(state, tracer, fn_state);
        };
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #() : ()
        let s_140_0: () = ();
        // S s_140_1: call EL2Enabled(s_140_0)
        let s_140_1: bool = EL2Enabled(state, tracer, s_140_0);
        // N s_140_2: branch s_140_1 b206 b141
        if s_140_1 {
            return block_206(state, tracer, fn_state);
        } else {
            return block_141(state, tracer, fn_state);
        };
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_141_0: const #0u : u8
        let s_141_0: bool = false;
        // D s_141_1: write-var gs#127667 <= s_141_0
        fn_state.gs_127667 = s_141_0;
        // N s_141_2: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_142_0: read-var gs#127667:u8
        let s_142_0: bool = fn_state.gs_127667;
        // N s_142_1: branch s_142_0 b205 b143
        if s_142_0 {
            return block_205(state, tracer, fn_state);
        } else {
            return block_143(state, tracer, fn_state);
        };
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_143_0: const #0u : u8
        let s_143_0: bool = false;
        // D s_143_1: write-var gs#127668 <= s_143_0
        fn_state.gs_127668 = s_143_0;
        // N s_143_2: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_144_0: read-var gs#127668:u8
        let s_144_0: bool = fn_state.gs_127668;
        // N s_144_1: branch s_144_0 b204 b145
        if s_144_0 {
            return block_204(state, tracer, fn_state);
        } else {
            return block_145(state, tracer, fn_state);
        };
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #() : ()
        let s_145_0: () = ();
        // S s_145_1: call EL2Enabled(s_145_0)
        let s_145_1: bool = EL2Enabled(state, tracer, s_145_0);
        // N s_145_2: branch s_145_1 b203 b146
        if s_145_1 {
            return block_203(state, tracer, fn_state);
        } else {
            return block_146(state, tracer, fn_state);
        };
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #0u : u8
        let s_146_0: bool = false;
        // D s_146_1: write-var gs#127669 <= s_146_0
        fn_state.gs_127669 = s_146_0;
        // N s_146_2: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_147_0: read-var gs#127669:u8
        let s_147_0: bool = fn_state.gs_127669;
        // N s_147_1: branch s_147_0 b202 b148
        if s_147_0 {
            return block_202(state, tracer, fn_state);
        } else {
            return block_148(state, tracer, fn_state);
        };
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_148_0: const #0u : u8
        let s_148_0: bool = false;
        // D s_148_1: write-var gs#127670 <= s_148_0
        fn_state.gs_127670 = s_148_0;
        // N s_148_2: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var gs#127670:u8
        let s_149_0: bool = fn_state.gs_127670;
        // N s_149_1: branch s_149_0 b201 b150
        if s_149_0 {
            return block_201(state, tracer, fn_state);
        } else {
            return block_150(state, tracer, fn_state);
        };
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #() : ()
        let s_150_0: () = ();
        // S s_150_1: call EL2Enabled(s_150_0)
        let s_150_1: bool = EL2Enabled(state, tracer, s_150_0);
        // N s_150_2: branch s_150_1 b200 b151
        if s_150_1 {
            return block_200(state, tracer, fn_state);
        } else {
            return block_151(state, tracer, fn_state);
        };
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #0u : u8
        let s_151_0: bool = false;
        // D s_151_1: write-var gs#127671 <= s_151_0
        fn_state.gs_127671 = s_151_0;
        // N s_151_2: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var gs#127671:u8
        let s_152_0: bool = fn_state.gs_127671;
        // N s_152_1: branch s_152_0 b199 b153
        if s_152_0 {
            return block_199(state, tracer, fn_state);
        } else {
            return block_153(state, tracer, fn_state);
        };
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #0u : u8
        let s_153_0: bool = false;
        // D s_153_1: write-var gs#127672 <= s_153_0
        fn_state.gs_127672 = s_153_0;
        // N s_153_2: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_154_0: read-var gs#127672:u8
        let s_154_0: bool = fn_state.gs_127672;
        // N s_154_1: branch s_154_0 b198 b155
        if s_154_0 {
            return block_198(state, tracer, fn_state);
        } else {
            return block_155(state, tracer, fn_state);
        };
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #424u : u32
        let s_155_0: u32 = 424;
        // D s_155_1: read-reg s_155_0:u8
        let s_155_1: u8 = {
            let value = state.read_register::<u8>(s_155_0 as isize);
            tracer.read_register(s_155_0 as isize, value);
            value
        };
        // C s_155_2: const #2u : u8
        let s_155_2: u8 = 2;
        // D s_155_3: cmp-lt s_155_1 s_155_2
        let s_155_3: bool = ((s_155_1) < (s_155_2));
        // N s_155_4: branch s_155_3 b197 b156
        if s_155_3 {
            return block_197(state, tracer, fn_state);
        } else {
            return block_156(state, tracer, fn_state);
        };
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_156_0: const #0u : u8
        let s_156_0: bool = false;
        // D s_156_1: write-var gs#127673 <= s_156_0
        fn_state.gs_127673 = s_156_0;
        // N s_156_2: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_157_0: read-var gs#127673:u8
        let s_157_0: bool = fn_state.gs_127673;
        // N s_157_1: branch s_157_0 b196 b158
        if s_157_0 {
            return block_196(state, tracer, fn_state);
        } else {
            return block_158(state, tracer, fn_state);
        };
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_158_0: const #0u : u8
        let s_158_0: bool = false;
        // D s_158_1: write-var gs#127674 <= s_158_0
        fn_state.gs_127674 = s_158_0;
        // N s_158_2: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_159_0: read-var gs#127674:u8
        let s_159_0: bool = fn_state.gs_127674;
        // N s_159_1: branch s_159_0 b190 b160
        if s_159_0 {
            return block_190(state, tracer, fn_state);
        } else {
            return block_160(state, tracer, fn_state);
        };
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_160_0: const #424u : u32
        let s_160_0: u32 = 424;
        // D s_160_1: read-reg s_160_0:u8
        let s_160_1: u8 = {
            let value = state.read_register::<u8>(s_160_0 as isize);
            tracer.read_register(s_160_0 as isize, value);
            value
        };
        // C s_160_2: const #2u : u8
        let s_160_2: u8 = 2;
        // D s_160_3: cmp-lt s_160_1 s_160_2
        let s_160_3: bool = ((s_160_1) < (s_160_2));
        // N s_160_4: branch s_160_3 b189 b161
        if s_160_3 {
            return block_189(state, tracer, fn_state);
        } else {
            return block_161(state, tracer, fn_state);
        };
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_161_0: const #0u : u8
        let s_161_0: bool = false;
        // D s_161_1: write-var gs#127675 <= s_161_0
        fn_state.gs_127675 = s_161_0;
        // N s_161_2: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_162_0: read-var gs#127675:u8
        let s_162_0: bool = fn_state.gs_127675;
        // N s_162_1: branch s_162_0 b188 b163
        if s_162_0 {
            return block_188(state, tracer, fn_state);
        } else {
            return block_163(state, tracer, fn_state);
        };
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #0u : u8
        let s_163_0: bool = false;
        // D s_163_1: write-var gs#127676 <= s_163_0
        fn_state.gs_127676 = s_163_0;
        // N s_163_2: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_164_0: read-var gs#127676:u8
        let s_164_0: bool = fn_state.gs_127676;
        // N s_164_1: branch s_164_0 b182 b165
        if s_164_0 {
            return block_182(state, tracer, fn_state);
        } else {
            return block_165(state, tracer, fn_state);
        };
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_165_0: const #424u : u32
        let s_165_0: u32 = 424;
        // D s_165_1: read-reg s_165_0:u8
        let s_165_1: u8 = {
            let value = state.read_register::<u8>(s_165_0 as isize);
            tracer.read_register(s_165_0 as isize, value);
            value
        };
        // C s_165_2: const #2u : u8
        let s_165_2: u8 = 2;
        // D s_165_3: cmp-lt s_165_1 s_165_2
        let s_165_3: bool = ((s_165_1) < (s_165_2));
        // N s_165_4: branch s_165_3 b181 b166
        if s_165_3 {
            return block_181(state, tracer, fn_state);
        } else {
            return block_166(state, tracer, fn_state);
        };
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #0u : u8
        let s_166_0: bool = false;
        // D s_166_1: write-var gs#127677 <= s_166_0
        fn_state.gs_127677 = s_166_0;
        // N s_166_2: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_167_0: read-var gs#127677:u8
        let s_167_0: bool = fn_state.gs_127677;
        // N s_167_1: branch s_167_0 b180 b168
        if s_167_0 {
            return block_180(state, tracer, fn_state);
        } else {
            return block_168(state, tracer, fn_state);
        };
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_168_0: const #0u : u8
        let s_168_0: bool = false;
        // D s_168_1: write-var gs#127678 <= s_168_0
        fn_state.gs_127678 = s_168_0;
        // N s_168_2: jump b169
        return block_169(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_169_0: read-var gs#127678:u8
        let s_169_0: bool = fn_state.gs_127678;
        // N s_169_1: branch s_169_0 b179 b170
        if s_169_0 {
            return block_179(state, tracer, fn_state);
        } else {
            return block_170(state, tracer, fn_state);
        };
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_170_0: const #0u : u8
        let s_170_0: bool = false;
        // D s_170_1: write-var gs#127679 <= s_170_0
        fn_state.gs_127679 = s_170_0;
        // N s_170_2: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_171_0: read-var gs#127679:u8
        let s_171_0: bool = fn_state.gs_127679;
        // N s_171_1: branch s_171_0 b173 b172
        if s_171_0 {
            return block_173(state, tracer, fn_state);
        } else {
            return block_172(state, tracer, fn_state);
        };
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_172_0: read-var t:i
        let s_172_0: i128 = fn_state.t;
        // D s_172_1: call R_read(s_172_0)
        let s_172_1: u32 = R_read(state, tracer, s_172_0);
        // D s_172_2: call ERXADDR2_write(s_172_1)
        let s_172_2: () = ERXADDR2_write(state, tracer, s_172_1);
        // N s_172_3: return
        return;
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_173_0: const #() : ()
        let s_173_0: () = ();
        // S s_173_1: call Halted(s_173_0)
        let s_173_1: bool = Halted(state, tracer, s_173_0);
        // N s_173_2: branch s_173_1 b178 b174
        if s_173_1 {
            return block_178(state, tracer, fn_state);
        } else {
            return block_174(state, tracer, fn_state);
        };
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_174_0: const #0u : u8
        let s_174_0: bool = false;
        // D s_174_1: write-var gs#127680 <= s_174_0
        fn_state.gs_127680 = s_174_0;
        // N s_174_2: jump b175
        return block_175(state, tracer, fn_state);
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_175_0: read-var gs#127680:u8
        let s_175_0: bool = fn_state.gs_127680;
        // N s_175_1: branch s_175_0 b177 b176
        if s_175_0 {
            return block_177(state, tracer, fn_state);
        } else {
            return block_176(state, tracer, fn_state);
        };
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_176_0: const #() : ()
        let s_176_0: () = ();
        // S s_176_1: call AArch32_TakeMonitorTrapException(s_176_0)
        let s_176_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_176_0);
        // N s_176_2: return
        return;
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_177_0: panic
        panic!("{:?}", ());
        // N s_177_1: return
        return;
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_178_0: const #() : ()
        let s_178_0: () = ();
        // S s_178_1: call EDSCR_read(s_178_0)
        let s_178_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_178_0);
        // S s_178_2: call _get_EDSCR_Type_SDD(s_178_1)
        let s_178_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_178_1);
        // S s_178_3: cast zx s_178_2 -> bv
        let s_178_3: Bits = Bits::new(s_178_2 as u128, 1u16);
        // C s_178_4: const #1u : u8
        let s_178_4: bool = true;
        // C s_178_5: cast zx s_178_4 -> bv
        let s_178_5: Bits = Bits::new(s_178_4 as u128, 1u16);
        // S s_178_6: cmp-eq s_178_3 s_178_5
        let s_178_6: bool = ((s_178_3) == (s_178_5));
        // D s_178_7: write-var gs#127680 <= s_178_6
        fn_state.gs_127680 = s_178_6;
        // N s_178_8: jump b175
        return block_175(state, tracer, fn_state);
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_179_0: read-var __SCR_TERR:u8
        let s_179_0: bool = fn_state.u__SCR_TERR;
        // D s_179_1: cast zx s_179_0 -> bv
        let s_179_1: Bits = Bits::new(s_179_0 as u128, 1u16);
        // C s_179_2: const #1u : u8
        let s_179_2: bool = true;
        // C s_179_3: cast zx s_179_2 -> bv
        let s_179_3: Bits = Bits::new(s_179_2 as u128, 1u16);
        // D s_179_4: cmp-eq s_179_1 s_179_3
        let s_179_4: bool = ((s_179_1) == (s_179_3));
        // D s_179_5: write-var gs#127679 <= s_179_4
        fn_state.gs_127679 = s_179_4;
        // N s_179_6: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_180_0: read-var __PSTATE_M:u8
        let s_180_0: u8 = fn_state.u__PSTATE_M;
        // D s_180_1: cast zx s_180_0 -> bv
        let s_180_1: Bits = Bits::new(s_180_0 as u128, 5u16);
        // C s_180_2: const #384u : u32
        let s_180_2: u32 = 384;
        // D s_180_3: read-reg s_180_2:u8
        let s_180_3: u8 = {
            let value = state.read_register::<u8>(s_180_2 as isize);
            tracer.read_register(s_180_2 as isize, value);
            value
        };
        // D s_180_4: cast zx s_180_3 -> bv
        let s_180_4: Bits = Bits::new(s_180_3 as u128, 5u16);
        // D s_180_5: cmp-ne s_180_1 s_180_4
        let s_180_5: bool = ((s_180_1) != (s_180_4));
        // D s_180_6: write-var gs#127678 <= s_180_5
        fn_state.gs_127678 = s_180_5;
        // N s_180_7: jump b169
        return block_169(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_181_0: const #424u : u32
        let s_181_0: u32 = 424;
        // D s_181_1: read-reg s_181_0:u8
        let s_181_1: u8 = {
            let value = state.read_register::<u8>(s_181_0 as isize);
            tracer.read_register(s_181_0 as isize, value);
            value
        };
        // D s_181_2: call ELUsingAArch32(s_181_1)
        let s_181_2: bool = ELUsingAArch32(state, tracer, s_181_1);
        // D s_181_3: write-var gs#127677 <= s_181_2
        fn_state.gs_127677 = s_181_2;
        // N s_181_4: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_182_0: const #() : ()
        let s_182_0: () = ();
        // S s_182_1: call Halted(s_182_0)
        let s_182_1: bool = Halted(state, tracer, s_182_0);
        // N s_182_2: branch s_182_1 b187 b183
        if s_182_1 {
            return block_187(state, tracer, fn_state);
        } else {
            return block_183(state, tracer, fn_state);
        };
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_183_0: const #0u : u8
        let s_183_0: bool = false;
        // D s_183_1: write-var gs#127681 <= s_183_0
        fn_state.gs_127681 = s_183_0;
        // N s_183_2: jump b184
        return block_184(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_184_0: read-var gs#127681:u8
        let s_184_0: bool = fn_state.gs_127681;
        // N s_184_1: branch s_184_0 b186 b185
        if s_184_0 {
            return block_186(state, tracer, fn_state);
        } else {
            return block_185(state, tracer, fn_state);
        };
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_185_0: const #3u : u8
        let s_185_0: u8 = 3;
        // C s_185_1: cast zx s_185_0 -> bv
        let s_185_1: Bits = Bits::new(s_185_0 as u128, 8u16);
        // C s_185_2: cast zx s_185_1 -> i
        let s_185_2: i128 = (s_185_1.value() as i128);
        // C s_185_3: cast reint s_185_2 -> i64
        let s_185_3: i64 = (s_185_2 as i64);
        // C s_185_4: cast zx s_185_3 -> i
        let s_185_4: i128 = (i128::try_from(s_185_3).unwrap());
        // C s_185_5: const #424u : u32
        let s_185_5: u32 = 424;
        // D s_185_6: read-reg s_185_5:u8
        let s_185_6: u8 = {
            let value = state.read_register::<u8>(s_185_5 as isize);
            tracer.read_register(s_185_5 as isize, value);
            value
        };
        // D s_185_7: call AArch64_AArch32SystemAccessTrap(s_185_6, s_185_4)
        let s_185_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_185_6,
            s_185_4,
        );
        // N s_185_8: return
        return;
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_186_0: panic
        panic!("{:?}", ());
        // N s_186_1: return
        return;
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #() : ()
        let s_187_0: () = ();
        // S s_187_1: call EDSCR_read(s_187_0)
        let s_187_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_187_0);
        // S s_187_2: call _get_EDSCR_Type_SDD(s_187_1)
        let s_187_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_187_1);
        // S s_187_3: cast zx s_187_2 -> bv
        let s_187_3: Bits = Bits::new(s_187_2 as u128, 1u16);
        // C s_187_4: const #1u : u8
        let s_187_4: bool = true;
        // C s_187_5: cast zx s_187_4 -> bv
        let s_187_5: Bits = Bits::new(s_187_4 as u128, 1u16);
        // S s_187_6: cmp-eq s_187_3 s_187_5
        let s_187_6: bool = ((s_187_3) == (s_187_5));
        // D s_187_7: write-var gs#127681 <= s_187_6
        fn_state.gs_127681 = s_187_6;
        // N s_187_8: jump b184
        return block_184(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_188_0: read-var __SCR_EL3_TWERR:u8
        let s_188_0: bool = fn_state.u__SCR_EL3_TWERR;
        // D s_188_1: cast zx s_188_0 -> bv
        let s_188_1: Bits = Bits::new(s_188_0 as u128, 1u16);
        // C s_188_2: const #1u : u8
        let s_188_2: bool = true;
        // C s_188_3: cast zx s_188_2 -> bv
        let s_188_3: Bits = Bits::new(s_188_2 as u128, 1u16);
        // D s_188_4: cmp-eq s_188_1 s_188_3
        let s_188_4: bool = ((s_188_1) == (s_188_3));
        // D s_188_5: write-var gs#127676 <= s_188_4
        fn_state.gs_127676 = s_188_4;
        // N s_188_6: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_189_0: const #424u : u32
        let s_189_0: u32 = 424;
        // D s_189_1: read-reg s_189_0:u8
        let s_189_1: u8 = {
            let value = state.read_register::<u8>(s_189_0 as isize);
            tracer.read_register(s_189_0 as isize, value);
            value
        };
        // D s_189_2: call ELUsingAArch32(s_189_1)
        let s_189_2: bool = ELUsingAArch32(state, tracer, s_189_1);
        // D s_189_3: not s_189_2
        let s_189_3: bool = !s_189_2;
        // D s_189_4: write-var gs#127675 <= s_189_3
        fn_state.gs_127675 = s_189_3;
        // N s_189_5: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_190_0: const #() : ()
        let s_190_0: () = ();
        // S s_190_1: call Halted(s_190_0)
        let s_190_1: bool = Halted(state, tracer, s_190_0);
        // N s_190_2: branch s_190_1 b195 b191
        if s_190_1 {
            return block_195(state, tracer, fn_state);
        } else {
            return block_191(state, tracer, fn_state);
        };
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_191_0: const #0u : u8
        let s_191_0: bool = false;
        // D s_191_1: write-var gs#127682 <= s_191_0
        fn_state.gs_127682 = s_191_0;
        // N s_191_2: jump b192
        return block_192(state, tracer, fn_state);
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_192_0: read-var gs#127682:u8
        let s_192_0: bool = fn_state.gs_127682;
        // N s_192_1: branch s_192_0 b194 b193
        if s_192_0 {
            return block_194(state, tracer, fn_state);
        } else {
            return block_193(state, tracer, fn_state);
        };
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_193_0: const #3u : u8
        let s_193_0: u8 = 3;
        // C s_193_1: cast zx s_193_0 -> bv
        let s_193_1: Bits = Bits::new(s_193_0 as u128, 8u16);
        // C s_193_2: cast zx s_193_1 -> i
        let s_193_2: i128 = (s_193_1.value() as i128);
        // C s_193_3: cast reint s_193_2 -> i64
        let s_193_3: i64 = (s_193_2 as i64);
        // C s_193_4: cast zx s_193_3 -> i
        let s_193_4: i128 = (i128::try_from(s_193_3).unwrap());
        // C s_193_5: const #424u : u32
        let s_193_5: u32 = 424;
        // D s_193_6: read-reg s_193_5:u8
        let s_193_6: u8 = {
            let value = state.read_register::<u8>(s_193_5 as isize);
            tracer.read_register(s_193_5 as isize, value);
            value
        };
        // D s_193_7: call AArch64_AArch32SystemAccessTrap(s_193_6, s_193_4)
        let s_193_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_193_6,
            s_193_4,
        );
        // N s_193_8: return
        return;
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_194_0: panic
        panic!("{:?}", ());
        // N s_194_1: return
        return;
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_195_0: const #() : ()
        let s_195_0: () = ();
        // S s_195_1: call EDSCR_read(s_195_0)
        let s_195_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_195_0);
        // S s_195_2: call _get_EDSCR_Type_SDD(s_195_1)
        let s_195_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_195_1);
        // S s_195_3: cast zx s_195_2 -> bv
        let s_195_3: Bits = Bits::new(s_195_2 as u128, 1u16);
        // C s_195_4: const #1u : u8
        let s_195_4: bool = true;
        // C s_195_5: cast zx s_195_4 -> bv
        let s_195_5: Bits = Bits::new(s_195_4 as u128, 1u16);
        // S s_195_6: cmp-eq s_195_3 s_195_5
        let s_195_6: bool = ((s_195_3) == (s_195_5));
        // D s_195_7: write-var gs#127682 <= s_195_6
        fn_state.gs_127682 = s_195_6;
        // N s_195_8: jump b192
        return block_192(state, tracer, fn_state);
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_196_0: read-var __SCR_EL3_TERR:u8
        let s_196_0: bool = fn_state.u__SCR_EL3_TERR;
        // D s_196_1: cast zx s_196_0 -> bv
        let s_196_1: Bits = Bits::new(s_196_0 as u128, 1u16);
        // C s_196_2: const #1u : u8
        let s_196_2: bool = true;
        // C s_196_3: cast zx s_196_2 -> bv
        let s_196_3: Bits = Bits::new(s_196_2 as u128, 1u16);
        // D s_196_4: cmp-eq s_196_1 s_196_3
        let s_196_4: bool = ((s_196_1) == (s_196_3));
        // D s_196_5: write-var gs#127674 <= s_196_4
        fn_state.gs_127674 = s_196_4;
        // N s_196_6: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_197_0: const #424u : u32
        let s_197_0: u32 = 424;
        // D s_197_1: read-reg s_197_0:u8
        let s_197_1: u8 = {
            let value = state.read_register::<u8>(s_197_0 as isize);
            tracer.read_register(s_197_0 as isize, value);
            value
        };
        // D s_197_2: call ELUsingAArch32(s_197_1)
        let s_197_2: bool = ELUsingAArch32(state, tracer, s_197_1);
        // D s_197_3: not s_197_2
        let s_197_3: bool = !s_197_2;
        // D s_197_4: write-var gs#127673 <= s_197_3
        fn_state.gs_127673 = s_197_3;
        // N s_197_5: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_198_0: const #3u : u8
        let s_198_0: u8 = 3;
        // C s_198_1: cast zx s_198_0 -> bv
        let s_198_1: Bits = Bits::new(s_198_0 as u128, 8u16);
        // C s_198_2: cast zx s_198_1 -> i
        let s_198_2: i128 = (s_198_1.value() as i128);
        // C s_198_3: cast reint s_198_2 -> i64
        let s_198_3: i64 = (s_198_2 as i64);
        // C s_198_4: cast zx s_198_3 -> i
        let s_198_4: i128 = (i128::try_from(s_198_3).unwrap());
        // S s_198_5: call AArch32_TakeHypTrapException(s_198_4)
        let s_198_5: () = AArch32_TakeHypTrapException(state, tracer, s_198_4);
        // N s_198_6: return
        return;
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_199_0: read-var __HCR2_TERR:u8
        let s_199_0: bool = fn_state.u__HCR2_TERR;
        // D s_199_1: cast zx s_199_0 -> bv
        let s_199_1: Bits = Bits::new(s_199_0 as u128, 1u16);
        // C s_199_2: const #1u : u8
        let s_199_2: bool = true;
        // C s_199_3: cast zx s_199_2 -> bv
        let s_199_3: Bits = Bits::new(s_199_2 as u128, 1u16);
        // D s_199_4: cmp-eq s_199_1 s_199_3
        let s_199_4: bool = ((s_199_1) == (s_199_3));
        // D s_199_5: write-var gs#127672 <= s_199_4
        fn_state.gs_127672 = s_199_4;
        // N s_199_6: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_200_0: const #432u : u32
        let s_200_0: u32 = 432;
        // D s_200_1: read-reg s_200_0:u8
        let s_200_1: u8 = {
            let value = state.read_register::<u8>(s_200_0 as isize);
            tracer.read_register(s_200_0 as isize, value);
            value
        };
        // D s_200_2: call ELUsingAArch32(s_200_1)
        let s_200_2: bool = ELUsingAArch32(state, tracer, s_200_1);
        // D s_200_3: write-var gs#127671 <= s_200_2
        fn_state.gs_127671 = s_200_2;
        // N s_200_4: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_201_0: const #3u : u8
        let s_201_0: u8 = 3;
        // C s_201_1: cast zx s_201_0 -> bv
        let s_201_1: Bits = Bits::new(s_201_0 as u128, 8u16);
        // C s_201_2: cast zx s_201_1 -> i
        let s_201_2: i128 = (s_201_1.value() as i128);
        // C s_201_3: cast reint s_201_2 -> i64
        let s_201_3: i64 = (s_201_2 as i64);
        // C s_201_4: cast zx s_201_3 -> i
        let s_201_4: i128 = (i128::try_from(s_201_3).unwrap());
        // C s_201_5: const #432u : u32
        let s_201_5: u32 = 432;
        // D s_201_6: read-reg s_201_5:u8
        let s_201_6: u8 = {
            let value = state.read_register::<u8>(s_201_5 as isize);
            tracer.read_register(s_201_5 as isize, value);
            value
        };
        // D s_201_7: call AArch64_AArch32SystemAccessTrap(s_201_6, s_201_4)
        let s_201_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_201_6,
            s_201_4,
        );
        // N s_201_8: return
        return;
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_202_0: read-var __HCR_EL2_TERR:u8
        let s_202_0: bool = fn_state.u__HCR_EL2_TERR;
        // D s_202_1: cast zx s_202_0 -> bv
        let s_202_1: Bits = Bits::new(s_202_0 as u128, 1u16);
        // C s_202_2: const #1u : u8
        let s_202_2: bool = true;
        // C s_202_3: cast zx s_202_2 -> bv
        let s_202_3: Bits = Bits::new(s_202_2 as u128, 1u16);
        // D s_202_4: cmp-eq s_202_1 s_202_3
        let s_202_4: bool = ((s_202_1) == (s_202_3));
        // D s_202_5: write-var gs#127670 <= s_202_4
        fn_state.gs_127670 = s_202_4;
        // N s_202_6: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_203_0: const #432u : u32
        let s_203_0: u32 = 432;
        // D s_203_1: read-reg s_203_0:u8
        let s_203_1: u8 = {
            let value = state.read_register::<u8>(s_203_0 as isize);
            tracer.read_register(s_203_0 as isize, value);
            value
        };
        // D s_203_2: call ELUsingAArch32(s_203_1)
        let s_203_2: bool = ELUsingAArch32(state, tracer, s_203_1);
        // D s_203_3: not s_203_2
        let s_203_3: bool = !s_203_2;
        // D s_203_4: write-var gs#127669 <= s_203_3
        fn_state.gs_127669 = s_203_3;
        // N s_203_5: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_204_0: const #3u : u8
        let s_204_0: u8 = 3;
        // C s_204_1: cast zx s_204_0 -> bv
        let s_204_1: Bits = Bits::new(s_204_0 as u128, 8u16);
        // C s_204_2: cast zx s_204_1 -> i
        let s_204_2: i128 = (s_204_1.value() as i128);
        // C s_204_3: cast reint s_204_2 -> i64
        let s_204_3: i64 = (s_204_2 as i64);
        // C s_204_4: cast zx s_204_3 -> i
        let s_204_4: i128 = (i128::try_from(s_204_3).unwrap());
        // S s_204_5: call AArch32_TakeHypTrapException(s_204_4)
        let s_204_5: () = AArch32_TakeHypTrapException(state, tracer, s_204_4);
        // N s_204_6: return
        return;
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_205_0: read-var __HSTR_T5:u8
        let s_205_0: bool = fn_state.u__HSTR_T5;
        // D s_205_1: cast zx s_205_0 -> bv
        let s_205_1: Bits = Bits::new(s_205_0 as u128, 1u16);
        // C s_205_2: const #1u : u8
        let s_205_2: bool = true;
        // C s_205_3: cast zx s_205_2 -> bv
        let s_205_3: Bits = Bits::new(s_205_2 as u128, 1u16);
        // D s_205_4: cmp-eq s_205_1 s_205_3
        let s_205_4: bool = ((s_205_1) == (s_205_3));
        // D s_205_5: write-var gs#127668 <= s_205_4
        fn_state.gs_127668 = s_205_4;
        // N s_205_6: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_206_0: const #432u : u32
        let s_206_0: u32 = 432;
        // D s_206_1: read-reg s_206_0:u8
        let s_206_1: u8 = {
            let value = state.read_register::<u8>(s_206_0 as isize);
            tracer.read_register(s_206_0 as isize, value);
            value
        };
        // D s_206_2: call ELUsingAArch32(s_206_1)
        let s_206_2: bool = ELUsingAArch32(state, tracer, s_206_1);
        // D s_206_3: write-var gs#127667 <= s_206_2
        fn_state.gs_127667 = s_206_2;
        // N s_206_4: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_207_0: const #3u : u8
        let s_207_0: u8 = 3;
        // C s_207_1: cast zx s_207_0 -> bv
        let s_207_1: Bits = Bits::new(s_207_0 as u128, 8u16);
        // C s_207_2: cast zx s_207_1 -> i
        let s_207_2: i128 = (s_207_1.value() as i128);
        // C s_207_3: cast reint s_207_2 -> i64
        let s_207_3: i64 = (s_207_2 as i64);
        // C s_207_4: cast zx s_207_3 -> i
        let s_207_4: i128 = (i128::try_from(s_207_3).unwrap());
        // C s_207_5: const #432u : u32
        let s_207_5: u32 = 432;
        // D s_207_6: read-reg s_207_5:u8
        let s_207_6: u8 = {
            let value = state.read_register::<u8>(s_207_5 as isize);
            tracer.read_register(s_207_5 as isize, value);
            value
        };
        // D s_207_7: call AArch64_AArch32SystemAccessTrap(s_207_6, s_207_4)
        let s_207_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_207_6,
            s_207_4,
        );
        // N s_207_8: return
        return;
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_208_0: read-var __HSTR_EL2_T5:u8
        let s_208_0: bool = fn_state.u__HSTR_EL2_T5;
        // D s_208_1: cast zx s_208_0 -> bv
        let s_208_1: Bits = Bits::new(s_208_0 as u128, 1u16);
        // C s_208_2: const #1u : u8
        let s_208_2: bool = true;
        // C s_208_3: cast zx s_208_2 -> bv
        let s_208_3: Bits = Bits::new(s_208_2 as u128, 1u16);
        // D s_208_4: cmp-eq s_208_1 s_208_3
        let s_208_4: bool = ((s_208_1) == (s_208_3));
        // D s_208_5: write-var gs#127666 <= s_208_4
        fn_state.gs_127666 = s_208_4;
        // N s_208_6: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_209_0: const #432u : u32
        let s_209_0: u32 = 432;
        // D s_209_1: read-reg s_209_0:u8
        let s_209_1: u8 = {
            let value = state.read_register::<u8>(s_209_0 as isize);
            tracer.read_register(s_209_0 as isize, value);
            value
        };
        // D s_209_2: call ELUsingAArch32(s_209_1)
        let s_209_2: bool = ELUsingAArch32(state, tracer, s_209_1);
        // D s_209_3: not s_209_2
        let s_209_3: bool = !s_209_2;
        // D s_209_4: write-var gs#127665 <= s_209_3
        fn_state.gs_127665 = s_209_3;
        // N s_209_5: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_210_0: panic
        panic!("{:?}", ());
        // N s_210_1: return
        return;
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_211_0: read-var __SCR_TERR:u8
        let s_211_0: bool = fn_state.u__SCR_TERR;
        // D s_211_1: cast zx s_211_0 -> bv
        let s_211_1: Bits = Bits::new(s_211_0 as u128, 1u16);
        // C s_211_2: const #1u : u8
        let s_211_2: bool = true;
        // C s_211_3: cast zx s_211_2 -> bv
        let s_211_3: Bits = Bits::new(s_211_2 as u128, 1u16);
        // D s_211_4: cmp-eq s_211_1 s_211_3
        let s_211_4: bool = ((s_211_1) == (s_211_3));
        // D s_211_5: write-var gs#127664 <= s_211_4
        fn_state.gs_127664 = s_211_4;
        // N s_211_6: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_212_0: const #424u : u32
        let s_212_0: u32 = 424;
        // D s_212_1: read-reg s_212_0:u8
        let s_212_1: u8 = {
            let value = state.read_register::<u8>(s_212_0 as isize);
            tracer.read_register(s_212_0 as isize, value);
            value
        };
        // D s_212_2: call ELUsingAArch32(s_212_1)
        let s_212_2: bool = ELUsingAArch32(state, tracer, s_212_1);
        // D s_212_3: write-var gs#127663 <= s_212_2
        fn_state.gs_127663 = s_212_2;
        // N s_212_4: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_213_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_213_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_213_1: call __IMPDEF_boolean(s_213_0)
        let s_213_1: bool = u__IMPDEF_boolean(state, tracer, s_213_0);
        // D s_213_2: write-var gs#127662 <= s_213_1
        fn_state.gs_127662 = s_213_1;
        // N s_213_3: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_214_0: const #() : ()
        let s_214_0: () = ();
        // S s_214_1: call EDSCR_read(s_214_0)
        let s_214_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_214_0);
        // S s_214_2: call _get_EDSCR_Type_SDD(s_214_1)
        let s_214_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_214_1);
        // S s_214_3: cast zx s_214_2 -> bv
        let s_214_3: Bits = Bits::new(s_214_2 as u128, 1u16);
        // C s_214_4: const #1u : u8
        let s_214_4: bool = true;
        // C s_214_5: cast zx s_214_4 -> bv
        let s_214_5: Bits = Bits::new(s_214_4 as u128, 1u16);
        // S s_214_6: cmp-eq s_214_3 s_214_5
        let s_214_6: bool = ((s_214_3) == (s_214_5));
        // D s_214_7: write-var gs#127661 <= s_214_6
        fn_state.gs_127661 = s_214_6;
        // N s_214_8: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_215_0: const #424u : u32
        let s_215_0: u32 = 424;
        // D s_215_1: read-reg s_215_0:u8
        let s_215_1: u8 = {
            let value = state.read_register::<u8>(s_215_0 as isize);
            tracer.read_register(s_215_0 as isize, value);
            value
        };
        // C s_215_2: const #2u : u8
        let s_215_2: u8 = 2;
        // D s_215_3: cmp-lt s_215_1 s_215_2
        let s_215_3: bool = ((s_215_1) < (s_215_2));
        // D s_215_4: write-var gs#127660 <= s_215_3
        fn_state.gs_127660 = s_215_3;
        // N s_215_5: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_216_0: panic
        panic!("{:?}", ());
        // N s_216_1: return
        return;
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_217_0: read-var __SCR_EL3_TWERR:u8
        let s_217_0: bool = fn_state.u__SCR_EL3_TWERR;
        // D s_217_1: cast zx s_217_0 -> bv
        let s_217_1: Bits = Bits::new(s_217_0 as u128, 1u16);
        // C s_217_2: const #1u : u8
        let s_217_2: bool = true;
        // C s_217_3: cast zx s_217_2 -> bv
        let s_217_3: Bits = Bits::new(s_217_2 as u128, 1u16);
        // D s_217_4: cmp-eq s_217_1 s_217_3
        let s_217_4: bool = ((s_217_1) == (s_217_3));
        // D s_217_5: write-var gs#127659 <= s_217_4
        fn_state.gs_127659 = s_217_4;
        // N s_217_6: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_218_0: const #424u : u32
        let s_218_0: u32 = 424;
        // D s_218_1: read-reg s_218_0:u8
        let s_218_1: u8 = {
            let value = state.read_register::<u8>(s_218_0 as isize);
            tracer.read_register(s_218_0 as isize, value);
            value
        };
        // D s_218_2: call ELUsingAArch32(s_218_1)
        let s_218_2: bool = ELUsingAArch32(state, tracer, s_218_1);
        // D s_218_3: not s_218_2
        let s_218_3: bool = !s_218_2;
        // D s_218_4: write-var gs#127658 <= s_218_3
        fn_state.gs_127658 = s_218_3;
        // N s_218_5: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_219_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_219_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_219_1: call __IMPDEF_boolean(s_219_0)
        let s_219_1: bool = u__IMPDEF_boolean(state, tracer, s_219_0);
        // D s_219_2: write-var gs#127657 <= s_219_1
        fn_state.gs_127657 = s_219_1;
        // N s_219_3: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_220_0: const #() : ()
        let s_220_0: () = ();
        // S s_220_1: call EDSCR_read(s_220_0)
        let s_220_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_220_0);
        // S s_220_2: call _get_EDSCR_Type_SDD(s_220_1)
        let s_220_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_220_1);
        // S s_220_3: cast zx s_220_2 -> bv
        let s_220_3: Bits = Bits::new(s_220_2 as u128, 1u16);
        // C s_220_4: const #1u : u8
        let s_220_4: bool = true;
        // C s_220_5: cast zx s_220_4 -> bv
        let s_220_5: Bits = Bits::new(s_220_4 as u128, 1u16);
        // S s_220_6: cmp-eq s_220_3 s_220_5
        let s_220_6: bool = ((s_220_3) == (s_220_5));
        // D s_220_7: write-var gs#127656 <= s_220_6
        fn_state.gs_127656 = s_220_6;
        // N s_220_8: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_221<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_221_0: const #424u : u32
        let s_221_0: u32 = 424;
        // D s_221_1: read-reg s_221_0:u8
        let s_221_1: u8 = {
            let value = state.read_register::<u8>(s_221_0 as isize);
            tracer.read_register(s_221_0 as isize, value);
            value
        };
        // C s_221_2: const #2u : u8
        let s_221_2: u8 = 2;
        // D s_221_3: cmp-lt s_221_1 s_221_2
        let s_221_3: bool = ((s_221_1) < (s_221_2));
        // D s_221_4: write-var gs#127655 <= s_221_3
        fn_state.gs_127655 = s_221_3;
        // N s_221_5: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_222_0: panic
        panic!("{:?}", ());
        // N s_222_1: return
        return;
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_223_0: read-var __SCR_EL3_TERR:u8
        let s_223_0: bool = fn_state.u__SCR_EL3_TERR;
        // D s_223_1: cast zx s_223_0 -> bv
        let s_223_1: Bits = Bits::new(s_223_0 as u128, 1u16);
        // C s_223_2: const #1u : u8
        let s_223_2: bool = true;
        // C s_223_3: cast zx s_223_2 -> bv
        let s_223_3: Bits = Bits::new(s_223_2 as u128, 1u16);
        // D s_223_4: cmp-eq s_223_1 s_223_3
        let s_223_4: bool = ((s_223_1) == (s_223_3));
        // D s_223_5: write-var gs#127654 <= s_223_4
        fn_state.gs_127654 = s_223_4;
        // N s_223_6: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_224_0: const #424u : u32
        let s_224_0: u32 = 424;
        // D s_224_1: read-reg s_224_0:u8
        let s_224_1: u8 = {
            let value = state.read_register::<u8>(s_224_0 as isize);
            tracer.read_register(s_224_0 as isize, value);
            value
        };
        // D s_224_2: call ELUsingAArch32(s_224_1)
        let s_224_2: bool = ELUsingAArch32(state, tracer, s_224_1);
        // D s_224_3: not s_224_2
        let s_224_3: bool = !s_224_2;
        // D s_224_4: write-var gs#127653 <= s_224_3
        fn_state.gs_127653 = s_224_3;
        // N s_224_5: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_225_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_225_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_225_1: call __IMPDEF_boolean(s_225_0)
        let s_225_1: bool = u__IMPDEF_boolean(state, tracer, s_225_0);
        // D s_225_2: write-var gs#127652 <= s_225_1
        fn_state.gs_127652 = s_225_1;
        // N s_225_3: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_226_0: const #() : ()
        let s_226_0: () = ();
        // S s_226_1: call EDSCR_read(s_226_0)
        let s_226_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_226_0);
        // S s_226_2: call _get_EDSCR_Type_SDD(s_226_1)
        let s_226_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_226_1);
        // S s_226_3: cast zx s_226_2 -> bv
        let s_226_3: Bits = Bits::new(s_226_2 as u128, 1u16);
        // C s_226_4: const #1u : u8
        let s_226_4: bool = true;
        // C s_226_5: cast zx s_226_4 -> bv
        let s_226_5: Bits = Bits::new(s_226_4 as u128, 1u16);
        // S s_226_6: cmp-eq s_226_3 s_226_5
        let s_226_6: bool = ((s_226_3) == (s_226_5));
        // D s_226_7: write-var gs#127651 <= s_226_6
        fn_state.gs_127651 = s_226_6;
        // N s_226_8: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_227<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_227_0: const #424u : u32
        let s_227_0: u32 = 424;
        // D s_227_1: read-reg s_227_0:u8
        let s_227_1: u8 = {
            let value = state.read_register::<u8>(s_227_0 as isize);
            tracer.read_register(s_227_0 as isize, value);
            value
        };
        // C s_227_2: const #2u : u8
        let s_227_2: u8 = 2;
        // D s_227_3: cmp-lt s_227_1 s_227_2
        let s_227_3: bool = ((s_227_1) < (s_227_2));
        // D s_227_4: write-var gs#127650 <= s_227_3
        fn_state.gs_127650 = s_227_3;
        // N s_227_5: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_228<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_228_0: panic
        panic!("{:?}", ());
        // N s_228_1: return
        return;
    }
}
