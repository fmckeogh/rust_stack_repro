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
use AArch32_TakeHypTrapException::*;
use Halted::*;
use u_get_ICC_SRE_EL3_Type_Enable::*;
use u__IMPDEF_boolean::*;
use Mk_ICC_HSRE_Type::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_HSTR_EL2_Type_T12::*;
use HSTR_read::*;
use R_read::*;
use ELUsingAArch32::*;
use u_get_ICC_MSRE_Type_Enable::*;
use ICC_HSRE_write::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use EDSCR_read::*;
use u_get_HSTR_Type_T12::*;
use common::*;
pub fn ICC_HSRE_SysRegWrite32_3352216733a79da7<T: Tracer>(
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
        gs_128711: bool,
        u__HSTR_EL2_T12: bool,
        gs_128714: bool,
        gs_128713: bool,
        gs_128719: bool,
        u__HSTR_T12: bool,
        gs_128716: bool,
        gs_128718: bool,
        u__ICC_MSRE_Enable: bool,
        gs_128715: bool,
        gs_128708: bool,
        gs_128712: bool,
        u__PSTATE_EL: u8,
        u__ICC_SRE_EL3_Enable: bool,
        gs_128709: bool,
        gs_128710: bool,
        gs_128717: bool,
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
        // C s_0_3: const #104936u : u32
        let s_0_3: u32 = 104936;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_HSTR_EL2_Type_T12(s_0_4)
        let s_0_5: bool = u_get_HSTR_EL2_Type_T12(state, tracer, s_0_4);
        // D s_0_6: write-var __HSTR_EL2_T12 <= s_0_5
        fn_state.u__HSTR_EL2_T12 = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call HSTR_read(s_0_7)
        let s_0_8: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_7);
        // S s_0_9: call _get_HSTR_Type_T12(s_0_8)
        let s_0_9: bool = u_get_HSTR_Type_T12(state, tracer, s_0_8);
        // D s_0_10: write-var __HSTR_T12 <= s_0_9
        fn_state.u__HSTR_T12 = s_0_9;
        // C s_0_11: const #10200u : u32
        let s_0_11: u32 = 10200;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_ICC_SRE_EL3_Type_Enable(s_0_12)
        let s_0_13: bool = u_get_ICC_SRE_EL3_Type_Enable(state, tracer, s_0_12);
        // D s_0_14: write-var __ICC_SRE_EL3_Enable <= s_0_13
        fn_state.u__ICC_SRE_EL3_Enable = s_0_13;
        // C s_0_15: const #19992u : u32
        let s_0_15: u32 = 19992;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_ICC_MSRE_Type_Enable(s_0_16)
        let s_0_17: bool = u_get_ICC_MSRE_Type_Enable(state, tracer, s_0_16);
        // D s_0_18: write-var __ICC_MSRE_Enable <= s_0_17
        fn_state.u__ICC_MSRE_Enable = s_0_17;
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
        // N s_0_25: branch s_0_24 b58 b1
        if s_0_24 {
            return block_58(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b41 b2
        if s_1_5 {
            return block_41(state, tracer, fn_state);
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
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call EL2Enabled(s_5_0)
        let s_5_1: bool = EL2Enabled(state, tracer, s_5_0);
        // S s_5_2: not s_5_1
        let s_5_2: bool = !s_5_1;
        // N s_5_3: branch s_5_2 b7 b6
        if s_5_2 {
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
        // D s_6_0: read-var t:i
        let s_6_0: i128 = fn_state.t;
        // D s_6_1: call R_read(s_6_0)
        let s_6_1: u32 = R_read(state, tracer, s_6_0);
        // D s_6_2: call Mk_ICC_HSRE_Type(s_6_1)
        let s_6_2: ProductType700c18a878c5601b = Mk_ICC_HSRE_Type(state, tracer, s_6_1);
        // D s_6_3: call ICC_HSRE_write(s_6_2)
        let s_6_3: () = ICC_HSRE_write(state, tracer, s_6_2);
        // N s_6_4: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
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
        // N s_8_2: branch s_8_1 b40 b9
        if s_8_1 {
            return block_40(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#128708 <= s_9_0
        fn_state.gs_128708 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#128708:u8
        let s_10_0: bool = fn_state.gs_128708;
        // N s_10_1: branch s_10_0 b39 b11
        if s_10_0 {
            return block_39(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#128709 <= s_11_0
        fn_state.gs_128709 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#128709:u8
        let s_12_0: bool = fn_state.gs_128709;
        // N s_12_1: branch s_12_0 b38 b13
        if s_12_0 {
            return block_38(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#128710 <= s_13_0
        fn_state.gs_128710 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#128710:u8
        let s_14_0: bool = fn_state.gs_128710;
        // N s_14_1: branch s_14_0 b37 b15
        if s_14_0 {
            return block_37(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#128711 <= s_15_0
        fn_state.gs_128711 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#128711:u8
        let s_16_0: bool = fn_state.gs_128711;
        // N s_16_1: branch s_16_0 b36 b17
        if s_16_0 {
            return block_36(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#128712 <= s_17_0
        fn_state.gs_128712 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#128712:u8
        let s_18_0: bool = fn_state.gs_128712;
        // N s_18_1: branch s_18_0 b35 b19
        if s_18_0 {
            return block_35(state, tracer, fn_state);
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
        // N s_19_4: branch s_19_3 b34 b20
        if s_19_3 {
            return block_34(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#128713 <= s_20_0
        fn_state.gs_128713 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#128713:u8
        let s_21_0: bool = fn_state.gs_128713;
        // N s_21_1: branch s_21_0 b33 b22
        if s_21_0 {
            return block_33(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#128714 <= s_22_0
        fn_state.gs_128714 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#128714:u8
        let s_23_0: bool = fn_state.gs_128714;
        // N s_23_1: branch s_23_0 b27 b24
        if s_23_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var __ICC_MSRE_Enable:u8
        let s_24_0: bool = fn_state.u__ICC_MSRE_Enable;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 1u16);
        // C s_24_2: const #0u : u8
        let s_24_2: bool = false;
        // C s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // D s_24_4: cmp-eq s_24_1 s_24_3
        let s_24_4: bool = ((s_24_1) == (s_24_3));
        // N s_24_5: branch s_24_4 b26 b25
        if s_24_4 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var t:i
        let s_25_0: i128 = fn_state.t;
        // D s_25_1: call R_read(s_25_0)
        let s_25_1: u32 = R_read(state, tracer, s_25_0);
        // D s_25_2: call Mk_ICC_HSRE_Type(s_25_1)
        let s_25_2: ProductType700c18a878c5601b = Mk_ICC_HSRE_Type(
            state,
            tracer,
            s_25_1,
        );
        // D s_25_3: call ICC_HSRE_write(s_25_2)
        let s_25_3: () = ICC_HSRE_write(state, tracer, s_25_2);
        // N s_25_4: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: panic
        panic!("{:?}", ());
        // N s_26_1: return
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
        // D s_28_1: write-var gs#128715 <= s_28_0
        fn_state.gs_128715 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#128715:u8
        let s_29_0: bool = fn_state.gs_128715;
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
        // C s_30_0: const #3u : u8
        let s_30_0: u8 = 3;
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
        // D s_32_7: write-var gs#128715 <= s_32_6
        fn_state.gs_128715 = s_32_6;
        // N s_32_8: jump b29
        return block_29(state, tracer, fn_state);
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
        // D s_33_5: write-var gs#128714 <= s_33_4
        fn_state.gs_128714 = s_33_4;
        // N s_33_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #424u : u32
        let s_34_0: u32 = 424;
        // D s_34_1: read-reg s_34_0:u8
        let s_34_1: u8 = {
            let value = state.read_register::<u8>(s_34_0 as isize);
            tracer.read_register(s_34_0 as isize, value);
            value
        };
        // D s_34_2: call ELUsingAArch32(s_34_1)
        let s_34_2: bool = ELUsingAArch32(state, tracer, s_34_1);
        // D s_34_3: not s_34_2
        let s_34_3: bool = !s_34_2;
        // D s_34_4: write-var gs#128713 <= s_34_3
        fn_state.gs_128713 = s_34_3;
        // N s_34_5: jump b21
        return block_21(state, tracer, fn_state);
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
        // D s_36_0: read-var __ICC_SRE_EL3_Enable:u8
        let s_36_0: bool = fn_state.u__ICC_SRE_EL3_Enable;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 1u16);
        // C s_36_2: const #0u : u8
        let s_36_2: bool = false;
        // C s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 1u16);
        // D s_36_4: cmp-eq s_36_1 s_36_3
        let s_36_4: bool = ((s_36_1) == (s_36_3));
        // D s_36_5: write-var gs#128712 <= s_36_4
        fn_state.gs_128712 = s_36_4;
        // N s_36_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #424u : u32
        let s_37_0: u32 = 424;
        // D s_37_1: read-reg s_37_0:u8
        let s_37_1: u8 = {
            let value = state.read_register::<u8>(s_37_0 as isize);
            tracer.read_register(s_37_0 as isize, value);
            value
        };
        // D s_37_2: call ELUsingAArch32(s_37_1)
        let s_37_2: bool = ELUsingAArch32(state, tracer, s_37_1);
        // D s_37_3: not s_37_2
        let s_37_3: bool = !s_37_2;
        // D s_37_4: write-var gs#128711 <= s_37_3
        fn_state.gs_128711 = s_37_3;
        // N s_37_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_38_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_38_1: call __IMPDEF_boolean(s_38_0)
        let s_38_1: bool = u__IMPDEF_boolean(state, tracer, s_38_0);
        // D s_38_2: write-var gs#128710 <= s_38_1
        fn_state.gs_128710 = s_38_1;
        // N s_38_3: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #() : ()
        let s_39_0: () = ();
        // S s_39_1: call EDSCR_read(s_39_0)
        let s_39_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_39_0);
        // S s_39_2: call _get_EDSCR_Type_SDD(s_39_1)
        let s_39_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_39_1);
        // S s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // C s_39_4: const #1u : u8
        let s_39_4: bool = true;
        // C s_39_5: cast zx s_39_4 -> bv
        let s_39_5: Bits = Bits::new(s_39_4 as u128, 1u16);
        // S s_39_6: cmp-eq s_39_3 s_39_5
        let s_39_6: bool = ((s_39_3) == (s_39_5));
        // D s_39_7: write-var gs#128709 <= s_39_6
        fn_state.gs_128709 = s_39_6;
        // N s_39_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #424u : u32
        let s_40_0: u32 = 424;
        // D s_40_1: read-reg s_40_0:u8
        let s_40_1: u8 = {
            let value = state.read_register::<u8>(s_40_0 as isize);
            tracer.read_register(s_40_0 as isize, value);
            value
        };
        // C s_40_2: const #2u : u8
        let s_40_2: u8 = 2;
        // D s_40_3: cmp-lt s_40_1 s_40_2
        let s_40_3: bool = ((s_40_1) < (s_40_2));
        // D s_40_4: write-var gs#128708 <= s_40_3
        fn_state.gs_128708 = s_40_3;
        // N s_40_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #() : ()
        let s_41_0: () = ();
        // S s_41_1: call EL2Enabled(s_41_0)
        let s_41_1: bool = EL2Enabled(state, tracer, s_41_0);
        // N s_41_2: branch s_41_1 b57 b42
        if s_41_1 {
            return block_57(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#128716 <= s_42_0
        fn_state.gs_128716 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#128716:u8
        let s_43_0: bool = fn_state.gs_128716;
        // N s_43_1: branch s_43_0 b56 b44
        if s_43_0 {
            return block_56(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#128717 <= s_44_0
        fn_state.gs_128717 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#128717:u8
        let s_45_0: bool = fn_state.gs_128717;
        // N s_45_1: branch s_45_0 b55 b46
        if s_45_0 {
            return block_55(state, tracer, fn_state);
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
        // N s_46_2: branch s_46_1 b54 b47
        if s_46_1 {
            return block_54(state, tracer, fn_state);
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
        // D s_47_1: write-var gs#128718 <= s_47_0
        fn_state.gs_128718 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#128718:u8
        let s_48_0: bool = fn_state.gs_128718;
        // N s_48_1: branch s_48_0 b53 b49
        if s_48_0 {
            return block_53(state, tracer, fn_state);
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
        // D s_49_1: write-var gs#128719 <= s_49_0
        fn_state.gs_128719 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#128719:u8
        let s_50_0: bool = fn_state.gs_128719;
        // N s_50_1: branch s_50_0 b52 b51
        if s_50_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_51_0: panic
        panic!("{:?}", ());
        // N s_51_1: return
        return;
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #3u : u8
        let s_52_0: u8 = 3;
        // C s_52_1: cast zx s_52_0 -> bv
        let s_52_1: Bits = Bits::new(s_52_0 as u128, 8u16);
        // C s_52_2: cast zx s_52_1 -> i
        let s_52_2: i128 = (s_52_1.value() as i128);
        // C s_52_3: cast reint s_52_2 -> i64
        let s_52_3: i64 = (s_52_2 as i64);
        // C s_52_4: cast zx s_52_3 -> i
        let s_52_4: i128 = (i128::try_from(s_52_3).unwrap());
        // S s_52_5: call AArch32_TakeHypTrapException(s_52_4)
        let s_52_5: () = AArch32_TakeHypTrapException(state, tracer, s_52_4);
        // N s_52_6: return
        return;
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var __HSTR_T12:u8
        let s_53_0: bool = fn_state.u__HSTR_T12;
        // D s_53_1: cast zx s_53_0 -> bv
        let s_53_1: Bits = Bits::new(s_53_0 as u128, 1u16);
        // C s_53_2: const #1u : u8
        let s_53_2: bool = true;
        // C s_53_3: cast zx s_53_2 -> bv
        let s_53_3: Bits = Bits::new(s_53_2 as u128, 1u16);
        // D s_53_4: cmp-eq s_53_1 s_53_3
        let s_53_4: bool = ((s_53_1) == (s_53_3));
        // D s_53_5: write-var gs#128719 <= s_53_4
        fn_state.gs_128719 = s_53_4;
        // N s_53_6: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #432u : u32
        let s_54_0: u32 = 432;
        // D s_54_1: read-reg s_54_0:u8
        let s_54_1: u8 = {
            let value = state.read_register::<u8>(s_54_0 as isize);
            tracer.read_register(s_54_0 as isize, value);
            value
        };
        // D s_54_2: call ELUsingAArch32(s_54_1)
        let s_54_2: bool = ELUsingAArch32(state, tracer, s_54_1);
        // D s_54_3: write-var gs#128718 <= s_54_2
        fn_state.gs_128718 = s_54_2;
        // N s_54_4: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #3u : u8
        let s_55_0: u8 = 3;
        // C s_55_1: cast zx s_55_0 -> bv
        let s_55_1: Bits = Bits::new(s_55_0 as u128, 8u16);
        // C s_55_2: cast zx s_55_1 -> i
        let s_55_2: i128 = (s_55_1.value() as i128);
        // C s_55_3: cast reint s_55_2 -> i64
        let s_55_3: i64 = (s_55_2 as i64);
        // C s_55_4: cast zx s_55_3 -> i
        let s_55_4: i128 = (i128::try_from(s_55_3).unwrap());
        // C s_55_5: const #432u : u32
        let s_55_5: u32 = 432;
        // D s_55_6: read-reg s_55_5:u8
        let s_55_6: u8 = {
            let value = state.read_register::<u8>(s_55_5 as isize);
            tracer.read_register(s_55_5 as isize, value);
            value
        };
        // D s_55_7: call AArch64_AArch32SystemAccessTrap(s_55_6, s_55_4)
        let s_55_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_55_6, s_55_4);
        // N s_55_8: return
        return;
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var __HSTR_EL2_T12:u8
        let s_56_0: bool = fn_state.u__HSTR_EL2_T12;
        // D s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 1u16);
        // C s_56_2: const #1u : u8
        let s_56_2: bool = true;
        // C s_56_3: cast zx s_56_2 -> bv
        let s_56_3: Bits = Bits::new(s_56_2 as u128, 1u16);
        // D s_56_4: cmp-eq s_56_1 s_56_3
        let s_56_4: bool = ((s_56_1) == (s_56_3));
        // D s_56_5: write-var gs#128717 <= s_56_4
        fn_state.gs_128717 = s_56_4;
        // N s_56_6: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #432u : u32
        let s_57_0: u32 = 432;
        // D s_57_1: read-reg s_57_0:u8
        let s_57_1: u8 = {
            let value = state.read_register::<u8>(s_57_0 as isize);
            tracer.read_register(s_57_0 as isize, value);
            value
        };
        // D s_57_2: call ELUsingAArch32(s_57_1)
        let s_57_2: bool = ELUsingAArch32(state, tracer, s_57_1);
        // D s_57_3: not s_57_2
        let s_57_3: bool = !s_57_2;
        // D s_57_4: write-var gs#128716 <= s_57_3
        fn_state.gs_128716 = s_57_3;
        // N s_57_5: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_58_0: panic
        panic!("{:?}", ());
        // N s_58_1: return
        return;
    }
}
