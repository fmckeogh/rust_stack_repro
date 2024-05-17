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
use Mk_AMEVCNTR0_Type::*;
use u_get_HSTR_EL2_Type_T0::*;
use IsHighestEL::*;
use R_read::*;
use u_get_HSTR_Type_T0::*;
use ELUsingAArch32::*;
use EL2Enabled::*;
use HSTR_read::*;
use AArch64_AArch32SystemAccessTrap::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn AMEVCNTR0_SysRegWrite64_125e5d1a241e0a34<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    coproc: u8,
    opc1: u8,
    CRm: u8,
    t: i128,
    t2: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_104463: bool,
        gs_104466: bool,
        gs_104460: bool,
        gs_104462: bool,
        gs_104457: bool,
        gs_104461: bool,
        u__HSTR_EL2_T0: bool,
        u__HSTR_T0: bool,
        gs_104467: bool,
        gs_104456: bool,
        el: u8,
        coproc: u8,
        opc1: u8,
        CRm: u8,
        t: i128,
        t2: i128,
    }
    let fn_state = FunctionState {
        el,
        coproc,
        opc1,
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
        // C s_0_0: const #104936u : u32
        let s_0_0: u32 = 104936;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_HSTR_EL2_Type_T0(s_0_1)
        let s_0_2: bool = u_get_HSTR_EL2_Type_T0(state, tracer, s_0_1);
        // D s_0_3: write-var __HSTR_EL2_T0 <= s_0_2
        fn_state.u__HSTR_EL2_T0 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call HSTR_read(s_0_4)
        let s_0_5: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_4);
        // S s_0_6: call _get_HSTR_Type_T0(s_0_5)
        let s_0_6: bool = u_get_HSTR_Type_T0(state, tracer, s_0_5);
        // D s_0_7: write-var __HSTR_T0 <= s_0_6
        fn_state.u__HSTR_T0 = s_0_6;
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
        // N s_0_15: branch s_0_14 b30 b1
        if s_0_14 {
            return block_30(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#104456 <= s_1_0
        fn_state.gs_104456 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#104456:u8
        let s_2_0: bool = fn_state.gs_104456;
        // N s_2_1: branch s_2_0 b29 b3
        if s_2_0 {
            return block_29(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#104457 <= s_3_0
        fn_state.gs_104457 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#104457:u8
        let s_4_0: bool = fn_state.gs_104457;
        // N s_4_1: branch s_4_0 b28 b5
        if s_4_0 {
            return block_28(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#104460 <= s_5_0
        fn_state.gs_104460 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#104460:u8
        let s_6_0: bool = fn_state.gs_104460;
        // N s_6_1: branch s_6_0 b27 b7
        if s_6_0 {
            return block_27(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#104461 <= s_7_0
        fn_state.gs_104461 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#104461:u8
        let s_8_0: bool = fn_state.gs_104461;
        // N s_8_1: branch s_8_0 b26 b9
        if s_8_0 {
            return block_26(state, tracer, fn_state);
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
        // N s_9_7: branch s_9_6 b25 b10
        if s_9_6 {
            return block_25(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#104462 <= s_10_0
        fn_state.gs_104462 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#104462:u8
        let s_11_0: bool = fn_state.gs_104462;
        // N s_11_1: branch s_11_0 b24 b12
        if s_11_0 {
            return block_24(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#104463 <= s_12_0
        fn_state.gs_104463 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#104463:u8
        let s_13_0: bool = fn_state.gs_104463;
        // N s_13_1: branch s_13_0 b23 b14
        if s_13_0 {
            return block_23(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#104466 <= s_14_0
        fn_state.gs_104466 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#104466:u8
        let s_15_0: bool = fn_state.gs_104466;
        // N s_15_1: branch s_15_0 b22 b16
        if s_15_0 {
            return block_22(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#104467 <= s_16_0
        fn_state.gs_104467 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#104467:u8
        let s_17_0: bool = fn_state.gs_104467;
        // N s_17_1: branch s_17_0 b21 b18
        if s_17_0 {
            return block_21(state, tracer, fn_state);
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
        // D s_18_2: call IsHighestEL(s_18_1)
        let s_18_2: bool = IsHighestEL(state, tracer, s_18_1);
        // N s_18_3: branch s_18_2 b20 b19
        if s_18_2 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: panic
        panic!("{:?}", ());
        // N s_19_1: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var t2:i
        let s_20_0: i128 = fn_state.t2;
        // D s_20_1: call R_read(s_20_0)
        let s_20_1: u32 = R_read(state, tracer, s_20_0);
        // D s_20_2: read-var t:i
        let s_20_2: i128 = fn_state.t;
        // D s_20_3: call R_read(s_20_2)
        let s_20_3: u32 = R_read(state, tracer, s_20_2);
        // D s_20_4: cast zx s_20_1 -> bv
        let s_20_4: Bits = Bits::new(s_20_1 as u128, 32u16);
        // D s_20_5: cast zx s_20_3 -> bv
        let s_20_5: Bits = Bits::new(s_20_3 as u128, 32u16);
        // D s_20_6: cast reint s_20_4 -> u128
        let s_20_6: u128 = (s_20_4.value() as u128);
        // D s_20_7: size-of s_20_4
        let s_20_7: u16 = s_20_4.length();
        // D s_20_8: cast reint s_20_5 -> u128
        let s_20_8: u128 = (s_20_5.value() as u128);
        // D s_20_9: size-of s_20_5
        let s_20_9: u16 = s_20_5.length();
        // D s_20_10: lsl s_20_6 s_20_9
        let s_20_10: u128 = s_20_6 << s_20_9;
        // D s_20_11: or s_20_10 s_20_8
        let s_20_11: u128 = ((s_20_10) | (s_20_8));
        // D s_20_12: add s_20_7 s_20_9
        let s_20_12: u16 = (s_20_7 + s_20_9);
        // D s_20_13: create-bits s_20_11 s_20_12
        let s_20_13: Bits = Bits::new(s_20_11, s_20_12);
        // D s_20_14: cast reint s_20_13 -> u64
        let s_20_14: u64 = (s_20_13.value() as u64);
        // D s_20_15: call Mk_AMEVCNTR0_Type(s_20_14)
        let s_20_15: ProductType5c790c8ef59cc8b2 = Mk_AMEVCNTR0_Type(
            state,
            tracer,
            s_20_14,
        );
        // C s_20_16: const #3s : i
        let s_20_16: i128 = 3;
        // C s_20_17: const #14624u : u32
        let s_20_17: u32 = 14624;
        // D s_20_18: read-reg s_20_17:[struct; 4]
        let s_20_18: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_20_17 as isize);
            tracer.read_register(s_20_17 as isize, value);
            value
        };
        // D s_20_19: mutate-element s_20_18[s_20_16] <= s_20_15
        let s_20_19: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_20_18.clone();
            local[(s_20_16) as usize] = s_20_15;
            local
        };
        // D s_20_20: cast cvt s_20_19 -> [struct; 0]
        let s_20_20: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_20_19,
        );
        // D s_20_21: cast cvt s_20_20 -> [struct; 4]
        let s_20_21: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_20_20);
            buf
        };
        // C s_20_22: const #14624u : u32
        let s_20_22: u32 = 14624;
        // N s_20_23: write-reg s_20_22 <= s_20_21
        let s_20_23: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_20_22 as isize, s_20_21);
            tracer.write_register(s_20_22 as isize, s_20_21);
        };
        // N s_20_24: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #4u : u8
        let s_21_0: u8 = 4;
        // C s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 8u16);
        // C s_21_2: cast zx s_21_1 -> i
        let s_21_2: i128 = (s_21_1.value() as i128);
        // C s_21_3: cast reint s_21_2 -> i64
        let s_21_3: i64 = (s_21_2 as i64);
        // C s_21_4: cast zx s_21_3 -> i
        let s_21_4: i128 = (i128::try_from(s_21_3).unwrap());
        // S s_21_5: call AArch32_TakeHypTrapException(s_21_4)
        let s_21_5: () = AArch32_TakeHypTrapException(state, tracer, s_21_4);
        // N s_21_6: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var __HSTR_T0:u8
        let s_22_0: bool = fn_state.u__HSTR_T0;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 1u16);
        // C s_22_2: const #1u : u8
        let s_22_2: bool = true;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 1u16);
        // D s_22_4: cmp-eq s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) == (s_22_3));
        // D s_22_5: write-var gs#104467 <= s_22_4
        fn_state.gs_104467 = s_22_4;
        // N s_22_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#104466 <= s_23_0
        fn_state.gs_104466 = s_23_0;
        // N s_23_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #432u : u32
        let s_24_0: u32 = 432;
        // D s_24_1: read-reg s_24_0:u8
        let s_24_1: u8 = {
            let value = state.read_register::<u8>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // D s_24_2: call ELUsingAArch32(s_24_1)
        let s_24_2: bool = ELUsingAArch32(state, tracer, s_24_1);
        // D s_24_3: write-var gs#104463 <= s_24_2
        fn_state.gs_104463 = s_24_2;
        // N s_24_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call EL2Enabled(s_25_0)
        let s_25_1: bool = EL2Enabled(state, tracer, s_25_0);
        // D s_25_2: write-var gs#104462 <= s_25_1
        fn_state.gs_104462 = s_25_1;
        // N s_25_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #4u : u8
        let s_26_0: u8 = 4;
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
        // D s_27_0: read-var __HSTR_EL2_T0:u8
        let s_27_0: bool = fn_state.u__HSTR_EL2_T0;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #1u : u8
        let s_27_2: bool = true;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#104461 <= s_27_4
        fn_state.gs_104461 = s_27_4;
        // N s_27_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#104460 <= s_28_0
        fn_state.gs_104460 = s_28_0;
        // N s_28_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #432u : u32
        let s_29_0: u32 = 432;
        // D s_29_1: read-reg s_29_0:u8
        let s_29_1: u8 = {
            let value = state.read_register::<u8>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: call ELUsingAArch32(s_29_1)
        let s_29_2: bool = ELUsingAArch32(state, tracer, s_29_1);
        // D s_29_3: not s_29_2
        let s_29_3: bool = !s_29_2;
        // D s_29_4: write-var gs#104457 <= s_29_3
        fn_state.gs_104457 = s_29_3;
        // N s_29_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call EL2Enabled(s_30_0)
        let s_30_1: bool = EL2Enabled(state, tracer, s_30_0);
        // D s_30_2: write-var gs#104456 <= s_30_1
        fn_state.gs_104456 = s_30_1;
        // N s_30_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
