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
use HRMR_write::*;
use AArch32_TakeHypTrapException::*;
use IsHighestEL::*;
use R_read::*;
use EL2Enabled::*;
use ELUsingAArch32::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_HSTR_EL2_Type_T12::*;
use HSTR_read::*;
use u_get_HSTR_Type_T12::*;
use Mk_HRMR_Type::*;
use common::*;
pub fn HRMR_SysRegWrite32_da3e01dd793ebd87<T: Tracer>(
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
        gs_128470: bool,
        gs_128464: bool,
        gs_128471: bool,
        gs_128466: bool,
        gs_128465: bool,
        gs_128468: bool,
        gs_128463: bool,
        u__HSTR_EL2_T12: bool,
        gs_128467: bool,
        gs_128469: bool,
        u__HSTR_T12: bool,
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
        // C s_0_0: const #104936u : u32
        let s_0_0: u32 = 104936;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_HSTR_EL2_Type_T12(s_0_1)
        let s_0_2: bool = u_get_HSTR_EL2_Type_T12(state, tracer, s_0_1);
        // D s_0_3: write-var __HSTR_EL2_T12 <= s_0_2
        fn_state.u__HSTR_EL2_T12 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call HSTR_read(s_0_4)
        let s_0_5: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_4);
        // S s_0_6: call _get_HSTR_Type_T12(s_0_5)
        let s_0_6: bool = u_get_HSTR_Type_T12(state, tracer, s_0_5);
        // D s_0_7: write-var __HSTR_T12 <= s_0_6
        fn_state.u__HSTR_T12 = s_0_6;
        // C s_0_8: const #16975u : u32
        let s_0_8: u32 = 16975;
        // D s_0_9: read-reg s_0_8:u8
        let s_0_9: u8 = {
            let value = state.read_register::<u8>(s_0_8 as isize);
            tracer.read_register(s_0_8 as isize, value);
            value
        };
        // D s_0_10: cast zx s_0_9 -> bv
        let s_0_10: Bits = Bits::new(s_0_9 as u128, 2u16);
        // C s_0_11: const #440u : u32
        let s_0_11: u32 = 440;
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
        // N s_0_15: branch s_0_14 b33 b1
        if s_0_14 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#128463 <= s_1_0
        fn_state.gs_128463 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#128463:u8
        let s_2_0: bool = fn_state.gs_128463;
        // N s_2_1: branch s_2_0 b32 b3
        if s_2_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#128464 <= s_3_0
        fn_state.gs_128464 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#128464:u8
        let s_4_0: bool = fn_state.gs_128464;
        // N s_4_1: branch s_4_0 b31 b5
        if s_4_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#128465 <= s_5_0
        fn_state.gs_128465 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#128465:u8
        let s_6_0: bool = fn_state.gs_128465;
        // N s_6_1: branch s_6_0 b30 b7
        if s_6_0 {
            return block_30(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#128466 <= s_7_0
        fn_state.gs_128466 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#128466:u8
        let s_8_0: bool = fn_state.gs_128466;
        // N s_8_1: branch s_8_0 b29 b9
        if s_8_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #16975u : u32
        let s_9_0: u32 = 16975;
        // D s_9_1: read-reg s_9_0:u8
        let s_9_1: u8 = {
            let value = state.read_register::<u8>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 2u16);
        // C s_9_3: const #440u : u32
        let s_9_3: u32 = 440;
        // D s_9_4: read-reg s_9_3:u8
        let s_9_4: u8 = {
            let value = state.read_register::<u8>(s_9_3 as isize);
            tracer.read_register(s_9_3 as isize, value);
            value
        };
        // D s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 2u16);
        // D s_9_6: cmp-eq s_9_2 s_9_5
        let s_9_6: bool = ((s_9_2) == (s_9_5));
        // N s_9_7: branch s_9_6 b28 b10
        if s_9_6 {
            return block_28(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#128467 <= s_10_0
        fn_state.gs_128467 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#128467:u8
        let s_11_0: bool = fn_state.gs_128467;
        // N s_11_1: branch s_11_0 b27 b12
        if s_11_0 {
            return block_27(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#128468 <= s_12_0
        fn_state.gs_128468 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#128468:u8
        let s_13_0: bool = fn_state.gs_128468;
        // N s_13_1: branch s_13_0 b26 b14
        if s_13_0 {
            return block_26(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#128469 <= s_14_0
        fn_state.gs_128469 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#128469:u8
        let s_15_0: bool = fn_state.gs_128469;
        // N s_15_1: branch s_15_0 b25 b16
        if s_15_0 {
            return block_25(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#128470 <= s_16_0
        fn_state.gs_128470 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#128470:u8
        let s_17_0: bool = fn_state.gs_128470;
        // N s_17_1: branch s_17_0 b24 b18
        if s_17_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #16975u : u32
        let s_18_0: u32 = 16975;
        // D s_18_1: read-reg s_18_0:u8
        let s_18_1: u8 = {
            let value = state.read_register::<u8>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: cast zx s_18_1 -> bv
        let s_18_2: Bits = Bits::new(s_18_1 as u128, 2u16);
        // C s_18_3: const #432u : u32
        let s_18_3: u32 = 432;
        // D s_18_4: read-reg s_18_3:u8
        let s_18_4: u8 = {
            let value = state.read_register::<u8>(s_18_3 as isize);
            tracer.read_register(s_18_3 as isize, value);
            value
        };
        // D s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 2u16);
        // D s_18_6: cmp-eq s_18_2 s_18_5
        let s_18_6: bool = ((s_18_2) == (s_18_5));
        // N s_18_7: branch s_18_6 b23 b19
        if s_18_6 {
            return block_23(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#128471 <= s_19_0
        fn_state.gs_128471 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#128471:u8
        let s_20_0: bool = fn_state.gs_128471;
        // N s_20_1: branch s_20_0 b22 b21
        if s_20_0 {
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
        // N s_21_0: panic
        panic!("{:?}", ());
        // N s_21_1: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var t:i
        let s_22_0: i128 = fn_state.t;
        // D s_22_1: call R_read(s_22_0)
        let s_22_1: u32 = R_read(state, tracer, s_22_0);
        // D s_22_2: call Mk_HRMR_Type(s_22_1)
        let s_22_2: ProductType700c18a878c5601b = Mk_HRMR_Type(state, tracer, s_22_1);
        // D s_22_3: call HRMR_write(s_22_2)
        let s_22_3: () = HRMR_write(state, tracer, s_22_2);
        // N s_22_4: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #432u : u32
        let s_23_0: u32 = 432;
        // D s_23_1: read-reg s_23_0:u8
        let s_23_1: u8 = {
            let value = state.read_register::<u8>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call IsHighestEL(s_23_1)
        let s_23_2: bool = IsHighestEL(state, tracer, s_23_1);
        // D s_23_3: write-var gs#128471 <= s_23_2
        fn_state.gs_128471 = s_23_2;
        // N s_23_4: jump b20
        return block_20(state, tracer, fn_state);
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
        // S s_24_5: call AArch32_TakeHypTrapException(s_24_4)
        let s_24_5: () = AArch32_TakeHypTrapException(state, tracer, s_24_4);
        // N s_24_6: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var __HSTR_T12:u8
        let s_25_0: bool = fn_state.u__HSTR_T12;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // C s_25_2: const #1u : u8
        let s_25_2: bool = true;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#128470 <= s_25_4
        fn_state.gs_128470 = s_25_4;
        // N s_25_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #432u : u32
        let s_26_0: u32 = 432;
        // D s_26_1: read-reg s_26_0:u8
        let s_26_1: u8 = {
            let value = state.read_register::<u8>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: call ELUsingAArch32(s_26_1)
        let s_26_2: bool = ELUsingAArch32(state, tracer, s_26_1);
        // D s_26_3: write-var gs#128469 <= s_26_2
        fn_state.gs_128469 = s_26_2;
        // N s_26_4: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #432u : u32
        let s_27_0: u32 = 432;
        // D s_27_1: read-reg s_27_0:u8
        let s_27_1: u8 = {
            let value = state.read_register::<u8>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: call IsHighestEL(s_27_1)
        let s_27_2: bool = IsHighestEL(state, tracer, s_27_1);
        // D s_27_3: write-var gs#128468 <= s_27_2
        fn_state.gs_128468 = s_27_2;
        // N s_27_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call EL2Enabled(s_28_0)
        let s_28_1: bool = EL2Enabled(state, tracer, s_28_0);
        // D s_28_2: write-var gs#128467 <= s_28_1
        fn_state.gs_128467 = s_28_1;
        // N s_28_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #3u : u8
        let s_29_0: u8 = 3;
        // C s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 8u16);
        // C s_29_2: cast zx s_29_1 -> i
        let s_29_2: i128 = (s_29_1.value() as i128);
        // C s_29_3: cast reint s_29_2 -> i64
        let s_29_3: i64 = (s_29_2 as i64);
        // C s_29_4: cast zx s_29_3 -> i
        let s_29_4: i128 = (i128::try_from(s_29_3).unwrap());
        // C s_29_5: const #432u : u32
        let s_29_5: u32 = 432;
        // D s_29_6: read-reg s_29_5:u8
        let s_29_6: u8 = {
            let value = state.read_register::<u8>(s_29_5 as isize);
            tracer.read_register(s_29_5 as isize, value);
            value
        };
        // D s_29_7: call AArch64_AArch32SystemAccessTrap(s_29_6, s_29_4)
        let s_29_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_29_6, s_29_4);
        // N s_29_8: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var __HSTR_EL2_T12:u8
        let s_30_0: bool = fn_state.u__HSTR_EL2_T12;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 1u16);
        // C s_30_2: const #1u : u8
        let s_30_2: bool = true;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // D s_30_4: cmp-eq s_30_1 s_30_3
        let s_30_4: bool = ((s_30_1) == (s_30_3));
        // D s_30_5: write-var gs#128466 <= s_30_4
        fn_state.gs_128466 = s_30_4;
        // N s_30_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #432u : u32
        let s_31_0: u32 = 432;
        // D s_31_1: read-reg s_31_0:u8
        let s_31_1: u8 = {
            let value = state.read_register::<u8>(s_31_0 as isize);
            tracer.read_register(s_31_0 as isize, value);
            value
        };
        // D s_31_2: call ELUsingAArch32(s_31_1)
        let s_31_2: bool = ELUsingAArch32(state, tracer, s_31_1);
        // D s_31_3: not s_31_2
        let s_31_3: bool = !s_31_2;
        // D s_31_4: write-var gs#128465 <= s_31_3
        fn_state.gs_128465 = s_31_3;
        // N s_31_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #432u : u32
        let s_32_0: u32 = 432;
        // D s_32_1: read-reg s_32_0:u8
        let s_32_1: u8 = {
            let value = state.read_register::<u8>(s_32_0 as isize);
            tracer.read_register(s_32_0 as isize, value);
            value
        };
        // D s_32_2: call IsHighestEL(s_32_1)
        let s_32_2: bool = IsHighestEL(state, tracer, s_32_1);
        // D s_32_3: write-var gs#128464 <= s_32_2
        fn_state.gs_128464 = s_32_2;
        // N s_32_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #() : ()
        let s_33_0: () = ();
        // S s_33_1: call EL2Enabled(s_33_0)
        let s_33_1: bool = EL2Enabled(state, tracer, s_33_0);
        // D s_33_2: write-var gs#128463 <= s_33_1
        fn_state.gs_128463 = s_33_1;
        // N s_33_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
