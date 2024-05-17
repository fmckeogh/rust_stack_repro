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
use u_get_HSTR_Type_T7::*;
use AArch64_AArch32SystemAccessTrap::*;
use AArch32_TakeHypTrapException::*;
use R_read::*;
use ELUsingAArch32::*;
use u_get_HSTR_EL2_Type_T7::*;
use u_get_SCR_EL3_Type_EEL2::*;
use EL2Enabled::*;
use HSTR_read::*;
use AArch32_AT::*;
use u_get_SCR_EL3_Type_NS::*;
use common::*;
pub fn ATS12NSOPR_SysRegWrite32_29b73e1bf71ad72e<T: Tracer>(
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
        u__SCR_EL3_NS: bool,
        u__HSTR_T7: bool,
        gs_124154: bool,
        gs_124153: bool,
        gs_124151: bool,
        gs_124156: bool,
        u__PSTATE_EL: u8,
        gs_124155: bool,
        gs_124152: bool,
        u__HSTR_EL2_T7: bool,
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
        // D s_0_5: call _get_HSTR_EL2_Type_T7(s_0_4)
        let s_0_5: bool = u_get_HSTR_EL2_Type_T7(state, tracer, s_0_4);
        // D s_0_6: write-var __HSTR_EL2_T7 <= s_0_5
        fn_state.u__HSTR_EL2_T7 = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call HSTR_read(s_0_7)
        let s_0_8: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_7);
        // S s_0_9: call _get_HSTR_Type_T7(s_0_8)
        let s_0_9: bool = u_get_HSTR_Type_T7(state, tracer, s_0_8);
        // D s_0_10: write-var __HSTR_T7 <= s_0_9
        fn_state.u__HSTR_T7 = s_0_9;
        // C s_0_11: const #90704u : u32
        let s_0_11: u32 = 90704;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_SCR_EL3_Type_NS(s_0_12)
        let s_0_13: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_0_12);
        // D s_0_14: write-var __SCR_EL3_NS <= s_0_13
        fn_state.u__SCR_EL3_NS = s_0_13;
        // D s_0_15: read-var __PSTATE_EL:u8
        let s_0_15: u8 = fn_state.u__PSTATE_EL;
        // D s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 2u16);
        // C s_0_17: const #448u : u32
        let s_0_17: u32 = 448;
        // D s_0_18: read-reg s_0_17:u8
        let s_0_18: u8 = {
            let value = state.read_register::<u8>(s_0_17 as isize);
            tracer.read_register(s_0_17 as isize, value);
            value
        };
        // D s_0_19: cast zx s_0_18 -> bv
        let s_0_19: Bits = Bits::new(s_0_18 as u128, 2u16);
        // D s_0_20: cmp-eq s_0_16 s_0_19
        let s_0_20: bool = ((s_0_16) == (s_0_19));
        // N s_0_21: branch s_0_20 b34 b1
        if s_0_20 {
            return block_34(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b7 b2
        if s_1_5 {
            return block_7(state, tracer, fn_state);
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
        // D s_5_0: read-var t:i
        let s_5_0: i128 = fn_state.t;
        // D s_5_1: call R_read(s_5_0)
        let s_5_1: u32 = R_read(state, tracer, s_5_0);
        // C s_5_2: const #1u : u32
        let s_5_2: u32 = 1;
        // C s_5_3: const #440u : u32
        let s_5_3: u32 = 440;
        // D s_5_4: read-reg s_5_3:u8
        let s_5_4: u8 = {
            let value = state.read_register::<u8>(s_5_3 as isize);
            tracer.read_register(s_5_3 as isize, value);
            value
        };
        // C s_5_5: const #0u : u32
        let s_5_5: u32 = 0;
        // D s_5_6: call AArch32_AT(s_5_1, s_5_2, s_5_4, s_5_5)
        let s_5_6: () = AArch32_AT(state, tracer, s_5_1, s_5_2, s_5_4, s_5_5);
        // N s_5_7: return
        return;
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
        // C s_6_2: const #1u : u32
        let s_6_2: u32 = 1;
        // C s_6_3: const #440u : u32
        let s_6_3: u32 = 440;
        // D s_6_4: read-reg s_6_3:u8
        let s_6_4: u8 = {
            let value = state.read_register::<u8>(s_6_3 as isize);
            tracer.read_register(s_6_3 as isize, value);
            value
        };
        // C s_6_5: const #0u : u32
        let s_6_5: u32 = 0;
        // D s_6_6: call AArch32_AT(s_6_1, s_6_2, s_6_4, s_6_5)
        let s_6_6: () = AArch32_AT(state, tracer, s_6_1, s_6_2, s_6_4, s_6_5);
        // N s_6_7: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call EL2Enabled(s_7_0)
        let s_7_1: bool = EL2Enabled(state, tracer, s_7_0);
        // N s_7_2: branch s_7_1 b33 b8
        if s_7_1 {
            return block_33(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#124151 <= s_8_0
        fn_state.gs_124151 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#124151:u8
        let s_9_0: bool = fn_state.gs_124151;
        // N s_9_1: branch s_9_0 b32 b10
        if s_9_0 {
            return block_32(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#124152 <= s_10_0
        fn_state.gs_124152 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#124152:u8
        let s_11_0: bool = fn_state.gs_124152;
        // N s_11_1: branch s_11_0 b31 b12
        if s_11_0 {
            return block_31(state, tracer, fn_state);
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
        // S s_12_1: call EL2Enabled(s_12_0)
        let s_12_1: bool = EL2Enabled(state, tracer, s_12_0);
        // N s_12_2: branch s_12_1 b30 b13
        if s_12_1 {
            return block_30(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#124153 <= s_13_0
        fn_state.gs_124153 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#124153:u8
        let s_14_0: bool = fn_state.gs_124153;
        // N s_14_1: branch s_14_0 b29 b15
        if s_14_0 {
            return block_29(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#124154 <= s_15_0
        fn_state.gs_124154 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#124154:u8
        let s_16_0: bool = fn_state.gs_124154;
        // N s_16_1: branch s_16_0 b28 b17
        if s_16_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #432u : u32
        let s_17_0: u32 = 432;
        // D s_17_1: read-reg s_17_0:u8
        let s_17_1: u8 = {
            let value = state.read_register::<u8>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: call ELUsingAArch32(s_17_1)
        let s_17_2: bool = ELUsingAArch32(state, tracer, s_17_1);
        // D s_17_3: not s_17_2
        let s_17_3: bool = !s_17_2;
        // N s_17_4: branch s_17_3 b27 b18
        if s_17_3 {
            return block_27(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#124155 <= s_18_0
        fn_state.gs_124155 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#124155:u8
        let s_19_0: bool = fn_state.gs_124155;
        // N s_19_1: branch s_19_0 b26 b20
        if s_19_0 {
            return block_26(state, tracer, fn_state);
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
        // D s_20_2: call ELUsingAArch32(s_20_1)
        let s_20_2: bool = ELUsingAArch32(state, tracer, s_20_1);
        // D s_20_3: not s_20_2
        let s_20_3: bool = !s_20_2;
        // N s_20_4: branch s_20_3 b25 b21
        if s_20_3 {
            return block_25(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#124156 <= s_21_0
        fn_state.gs_124156 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#124156:u8
        let s_22_0: bool = fn_state.gs_124156;
        // N s_22_1: branch s_22_0 b24 b23
        if s_22_0 {
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
        // N s_23_0: panic
        panic!("{:?}", ());
        // N s_23_1: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #3u : u8
        let s_24_0: u8 = 3;
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
        // D s_24_7: call AArch64_AArch32SystemAccessTrap(s_24_6, s_24_4)
        let s_24_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_24_6, s_24_4);
        // N s_24_8: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var __SCR_EL3_NS:u8
        let s_25_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // C s_25_2: const #0u : u8
        let s_25_2: bool = false;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#124156 <= s_25_4
        fn_state.gs_124156 = s_25_4;
        // N s_25_6: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #3u : u8
        let s_26_0: u8 = 3;
        // C s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 8u16);
        // C s_26_2: cast zx s_26_1 -> i
        let s_26_2: i128 = (s_26_1.value() as i128);
        // C s_26_3: cast reint s_26_2 -> i64
        let s_26_3: i64 = (s_26_2 as i64);
        // C s_26_4: cast zx s_26_3 -> i
        let s_26_4: i128 = (i128::try_from(s_26_3).unwrap());
        // C s_26_5: const #432u : u32
        let s_26_5: u32 = 432;
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
        // C s_27_0: const #90704u : u32
        let s_27_0: u32 = 90704;
        // D s_27_1: read-reg s_27_0:struct
        let s_27_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: call _get_SCR_EL3_Type_NS(s_27_1)
        let s_27_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_27_1);
        // C s_27_3: const #90704u : u32
        let s_27_3: u32 = 90704;
        // D s_27_4: read-reg s_27_3:struct
        let s_27_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_27_3 as isize);
            tracer.read_register(s_27_3 as isize, value);
            value
        };
        // D s_27_5: call _get_SCR_EL3_Type_EEL2(s_27_4)
        let s_27_5: bool = u_get_SCR_EL3_Type_EEL2(state, tracer, s_27_4);
        // D s_27_6: cast zx s_27_2 -> bv
        let s_27_6: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_7: cast zx s_27_5 -> bv
        let s_27_7: Bits = Bits::new(s_27_5 as u128, 1u16);
        // D s_27_8: cast reint s_27_6 -> u128
        let s_27_8: u128 = (s_27_6.value() as u128);
        // D s_27_9: size-of s_27_6
        let s_27_9: u16 = s_27_6.length();
        // D s_27_10: cast reint s_27_7 -> u128
        let s_27_10: u128 = (s_27_7.value() as u128);
        // D s_27_11: size-of s_27_7
        let s_27_11: u16 = s_27_7.length();
        // D s_27_12: lsl s_27_8 s_27_11
        let s_27_12: u128 = s_27_8 << s_27_11;
        // D s_27_13: or s_27_12 s_27_10
        let s_27_13: u128 = ((s_27_12) | (s_27_10));
        // D s_27_14: add s_27_9 s_27_11
        let s_27_14: u16 = (s_27_9 + s_27_11);
        // D s_27_15: create-bits s_27_13 s_27_14
        let s_27_15: Bits = Bits::new(s_27_13, s_27_14);
        // D s_27_16: cast reint s_27_15 -> u8
        let s_27_16: u8 = (s_27_15.value() as u8);
        // D s_27_17: cast zx s_27_16 -> bv
        let s_27_17: Bits = Bits::new(s_27_16 as u128, 2u16);
        // C s_27_18: const #1u : u8
        let s_27_18: u8 = 1;
        // C s_27_19: cast zx s_27_18 -> bv
        let s_27_19: Bits = Bits::new(s_27_18 as u128, 2u16);
        // D s_27_20: cmp-eq s_27_17 s_27_19
        let s_27_20: bool = ((s_27_17) == (s_27_19));
        // D s_27_21: write-var gs#124155 <= s_27_20
        fn_state.gs_124155 = s_27_20;
        // N s_27_22: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #3u : u8
        let s_28_0: u8 = 3;
        // C s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 8u16);
        // C s_28_2: cast zx s_28_1 -> i
        let s_28_2: i128 = (s_28_1.value() as i128);
        // C s_28_3: cast reint s_28_2 -> i64
        let s_28_3: i64 = (s_28_2 as i64);
        // C s_28_4: cast zx s_28_3 -> i
        let s_28_4: i128 = (i128::try_from(s_28_3).unwrap());
        // S s_28_5: call AArch32_TakeHypTrapException(s_28_4)
        let s_28_5: () = AArch32_TakeHypTrapException(state, tracer, s_28_4);
        // N s_28_6: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var __HSTR_T7:u8
        let s_29_0: bool = fn_state.u__HSTR_T7;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #1u : u8
        let s_29_2: bool = true;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#124154 <= s_29_4
        fn_state.gs_124154 = s_29_4;
        // N s_29_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #432u : u32
        let s_30_0: u32 = 432;
        // D s_30_1: read-reg s_30_0:u8
        let s_30_1: u8 = {
            let value = state.read_register::<u8>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // D s_30_2: call ELUsingAArch32(s_30_1)
        let s_30_2: bool = ELUsingAArch32(state, tracer, s_30_1);
        // D s_30_3: write-var gs#124153 <= s_30_2
        fn_state.gs_124153 = s_30_2;
        // N s_30_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #3u : u8
        let s_31_0: u8 = 3;
        // C s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 8u16);
        // C s_31_2: cast zx s_31_1 -> i
        let s_31_2: i128 = (s_31_1.value() as i128);
        // C s_31_3: cast reint s_31_2 -> i64
        let s_31_3: i64 = (s_31_2 as i64);
        // C s_31_4: cast zx s_31_3 -> i
        let s_31_4: i128 = (i128::try_from(s_31_3).unwrap());
        // C s_31_5: const #432u : u32
        let s_31_5: u32 = 432;
        // D s_31_6: read-reg s_31_5:u8
        let s_31_6: u8 = {
            let value = state.read_register::<u8>(s_31_5 as isize);
            tracer.read_register(s_31_5 as isize, value);
            value
        };
        // D s_31_7: call AArch64_AArch32SystemAccessTrap(s_31_6, s_31_4)
        let s_31_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_31_6, s_31_4);
        // N s_31_8: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var __HSTR_EL2_T7:u8
        let s_32_0: bool = fn_state.u__HSTR_EL2_T7;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // C s_32_2: const #1u : u8
        let s_32_2: bool = true;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // D s_32_5: write-var gs#124152 <= s_32_4
        fn_state.gs_124152 = s_32_4;
        // N s_32_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #432u : u32
        let s_33_0: u32 = 432;
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
        // D s_33_4: write-var gs#124151 <= s_33_3
        fn_state.gs_124151 = s_33_3;
        // N s_33_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_34_0: panic
        panic!("{:?}", ());
        // N s_34_1: return
        return;
    }
}
