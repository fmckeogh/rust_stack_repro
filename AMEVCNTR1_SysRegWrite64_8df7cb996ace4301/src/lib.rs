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
use u_get_HSTR_Type_T5::*;
use Mk_AMEVCNTR1_Type::*;
use IsHighestEL::*;
use R_read::*;
use AMEVCNTR1_set::*;
use AArch32_TakeHypTrapException::*;
use ELUsingAArch32::*;
use u_get_HSTR_EL2_Type_T5::*;
use EL2Enabled::*;
use HSTR_read::*;
use AArch64_AArch32SystemAccessTrap::*;
use IsG1ActivityMonitorImplemented::*;
use common::*;
pub fn AMEVCNTR1_SysRegWrite64_8df7cb996ace4301<T: Tracer>(
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
        u__HSTR_EL2_T5: bool,
        gs_104663: bool,
        gs_104662: bool,
        gs_104667: bool,
        gs_104672: bool,
        gs_104673: bool,
        gs_104666: bool,
        gs_104669: bool,
        u__HSTR_T5: bool,
        gs_104668: bool,
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
        // D s_0_2: call _get_HSTR_EL2_Type_T5(s_0_1)
        let s_0_2: bool = u_get_HSTR_EL2_Type_T5(state, tracer, s_0_1);
        // D s_0_3: write-var __HSTR_EL2_T5 <= s_0_2
        fn_state.u__HSTR_EL2_T5 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call HSTR_read(s_0_4)
        let s_0_5: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_4);
        // S s_0_6: call _get_HSTR_Type_T5(s_0_5)
        let s_0_6: bool = u_get_HSTR_Type_T5(state, tracer, s_0_5);
        // D s_0_7: write-var __HSTR_T5 <= s_0_6
        fn_state.u__HSTR_T5 = s_0_6;
        // C s_0_8: const #11s : i
        let s_0_8: i128 = 11;
        // S s_0_9: call IsG1ActivityMonitorImplemented(s_0_8)
        let s_0_9: bool = IsG1ActivityMonitorImplemented(state, tracer, s_0_8);
        // S s_0_10: not s_0_9
        let s_0_10: bool = !s_0_9;
        // N s_0_11: branch s_0_10 b32 b1
        if s_0_10 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #440u : u32
        let s_1_3: u32 = 440;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // D s_1_6: cmp-eq s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) == (s_1_5));
        // N s_1_7: branch s_1_6 b31 b2
        if s_1_6 {
            return block_31(state, tracer, fn_state);
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
        // D s_2_1: write-var gs#104662 <= s_2_0
        fn_state.gs_104662 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#104662:u8
        let s_3_0: bool = fn_state.gs_104662;
        // N s_3_1: branch s_3_0 b30 b4
        if s_3_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#104663 <= s_4_0
        fn_state.gs_104663 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#104663:u8
        let s_5_0: bool = fn_state.gs_104663;
        // N s_5_1: branch s_5_0 b29 b6
        if s_5_0 {
            return block_29(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#104666 <= s_6_0
        fn_state.gs_104666 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#104666:u8
        let s_7_0: bool = fn_state.gs_104666;
        // N s_7_1: branch s_7_0 b28 b8
        if s_7_0 {
            return block_28(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#104667 <= s_8_0
        fn_state.gs_104667 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#104667:u8
        let s_9_0: bool = fn_state.gs_104667;
        // N s_9_1: branch s_9_0 b27 b10
        if s_9_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #16975u : u32
        let s_10_0: u32 = 16975;
        // D s_10_1: read-reg s_10_0:u8
        let s_10_1: u8 = {
            let value = state.read_register::<u8>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 2u16);
        // C s_10_3: const #440u : u32
        let s_10_3: u32 = 440;
        // D s_10_4: read-reg s_10_3:u8
        let s_10_4: u8 = {
            let value = state.read_register::<u8>(s_10_3 as isize);
            tracer.read_register(s_10_3 as isize, value);
            value
        };
        // D s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 2u16);
        // D s_10_6: cmp-eq s_10_2 s_10_5
        let s_10_6: bool = ((s_10_2) == (s_10_5));
        // N s_10_7: branch s_10_6 b26 b11
        if s_10_6 {
            return block_26(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#104668 <= s_11_0
        fn_state.gs_104668 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#104668:u8
        let s_12_0: bool = fn_state.gs_104668;
        // N s_12_1: branch s_12_0 b25 b13
        if s_12_0 {
            return block_25(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#104669 <= s_13_0
        fn_state.gs_104669 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#104669:u8
        let s_14_0: bool = fn_state.gs_104669;
        // N s_14_1: branch s_14_0 b24 b15
        if s_14_0 {
            return block_24(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#104672 <= s_15_0
        fn_state.gs_104672 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#104672:u8
        let s_16_0: bool = fn_state.gs_104672;
        // N s_16_1: branch s_16_0 b23 b17
        if s_16_0 {
            return block_23(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#104673 <= s_17_0
        fn_state.gs_104673 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#104673:u8
        let s_18_0: bool = fn_state.gs_104673;
        // N s_18_1: branch s_18_0 b22 b19
        if s_18_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #16975u : u32
        let s_19_0: u32 = 16975;
        // D s_19_1: read-reg s_19_0:u8
        let s_19_1: u8 = {
            let value = state.read_register::<u8>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call IsHighestEL(s_19_1)
        let s_19_2: bool = IsHighestEL(state, tracer, s_19_1);
        // N s_19_3: branch s_19_2 b21 b20
        if s_19_2 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: panic
        panic!("{:?}", ());
        // N s_20_1: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var t2:i
        let s_21_0: i128 = fn_state.t2;
        // D s_21_1: call R_read(s_21_0)
        let s_21_1: u32 = R_read(state, tracer, s_21_0);
        // D s_21_2: read-var t:i
        let s_21_2: i128 = fn_state.t;
        // D s_21_3: call R_read(s_21_2)
        let s_21_3: u32 = R_read(state, tracer, s_21_2);
        // D s_21_4: cast zx s_21_1 -> bv
        let s_21_4: Bits = Bits::new(s_21_1 as u128, 32u16);
        // D s_21_5: cast zx s_21_3 -> bv
        let s_21_5: Bits = Bits::new(s_21_3 as u128, 32u16);
        // D s_21_6: cast reint s_21_4 -> u128
        let s_21_6: u128 = (s_21_4.value() as u128);
        // D s_21_7: size-of s_21_4
        let s_21_7: u16 = s_21_4.length();
        // D s_21_8: cast reint s_21_5 -> u128
        let s_21_8: u128 = (s_21_5.value() as u128);
        // D s_21_9: size-of s_21_5
        let s_21_9: u16 = s_21_5.length();
        // D s_21_10: lsl s_21_6 s_21_9
        let s_21_10: u128 = s_21_6 << s_21_9;
        // D s_21_11: or s_21_10 s_21_8
        let s_21_11: u128 = ((s_21_10) | (s_21_8));
        // D s_21_12: add s_21_7 s_21_9
        let s_21_12: u16 = (s_21_7 + s_21_9);
        // D s_21_13: create-bits s_21_11 s_21_12
        let s_21_13: Bits = Bits::new(s_21_11, s_21_12);
        // D s_21_14: cast reint s_21_13 -> u64
        let s_21_14: u64 = (s_21_13.value() as u64);
        // D s_21_15: call Mk_AMEVCNTR1_Type(s_21_14)
        let s_21_15: ProductType5c790c8ef59cc8b2 = Mk_AMEVCNTR1_Type(
            state,
            tracer,
            s_21_14,
        );
        // C s_21_16: const #11s : i64
        let s_21_16: i64 = 11;
        // D s_21_17: call AMEVCNTR1_set(s_21_16, s_21_15)
        let s_21_17: () = AMEVCNTR1_set(state, tracer, s_21_16, s_21_15);
        // N s_21_18: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #4u : u8
        let s_22_0: u8 = 4;
        // C s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 8u16);
        // C s_22_2: cast zx s_22_1 -> i
        let s_22_2: i128 = (s_22_1.value() as i128);
        // C s_22_3: cast reint s_22_2 -> i64
        let s_22_3: i64 = (s_22_2 as i64);
        // C s_22_4: cast zx s_22_3 -> i
        let s_22_4: i128 = (i128::try_from(s_22_3).unwrap());
        // S s_22_5: call AArch32_TakeHypTrapException(s_22_4)
        let s_22_5: () = AArch32_TakeHypTrapException(state, tracer, s_22_4);
        // N s_22_6: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var __HSTR_T5:u8
        let s_23_0: bool = fn_state.u__HSTR_T5;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 1u16);
        // C s_23_2: const #1u : u8
        let s_23_2: bool = true;
        // C s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // D s_23_4: cmp-eq s_23_1 s_23_3
        let s_23_4: bool = ((s_23_1) == (s_23_3));
        // D s_23_5: write-var gs#104673 <= s_23_4
        fn_state.gs_104673 = s_23_4;
        // N s_23_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#104672 <= s_24_0
        fn_state.gs_104672 = s_24_0;
        // N s_24_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #432u : u32
        let s_25_0: u32 = 432;
        // D s_25_1: read-reg s_25_0:u8
        let s_25_1: u8 = {
            let value = state.read_register::<u8>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: call ELUsingAArch32(s_25_1)
        let s_25_2: bool = ELUsingAArch32(state, tracer, s_25_1);
        // D s_25_3: write-var gs#104669 <= s_25_2
        fn_state.gs_104669 = s_25_2;
        // N s_25_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call EL2Enabled(s_26_0)
        let s_26_1: bool = EL2Enabled(state, tracer, s_26_0);
        // D s_26_2: write-var gs#104668 <= s_26_1
        fn_state.gs_104668 = s_26_1;
        // N s_26_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #4u : u8
        let s_27_0: u8 = 4;
        // C s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 8u16);
        // C s_27_2: cast zx s_27_1 -> i
        let s_27_2: i128 = (s_27_1.value() as i128);
        // C s_27_3: cast reint s_27_2 -> i64
        let s_27_3: i64 = (s_27_2 as i64);
        // C s_27_4: cast zx s_27_3 -> i
        let s_27_4: i128 = (i128::try_from(s_27_3).unwrap());
        // C s_27_5: const #432u : u32
        let s_27_5: u32 = 432;
        // D s_27_6: read-reg s_27_5:u8
        let s_27_6: u8 = {
            let value = state.read_register::<u8>(s_27_5 as isize);
            tracer.read_register(s_27_5 as isize, value);
            value
        };
        // D s_27_7: call AArch64_AArch32SystemAccessTrap(s_27_6, s_27_4)
        let s_27_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_27_6, s_27_4);
        // N s_27_8: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var __HSTR_EL2_T5:u8
        let s_28_0: bool = fn_state.u__HSTR_EL2_T5;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 1u16);
        // C s_28_2: const #1u : u8
        let s_28_2: bool = true;
        // C s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // D s_28_4: cmp-eq s_28_1 s_28_3
        let s_28_4: bool = ((s_28_1) == (s_28_3));
        // D s_28_5: write-var gs#104667 <= s_28_4
        fn_state.gs_104667 = s_28_4;
        // N s_28_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#104666 <= s_29_0
        fn_state.gs_104666 = s_29_0;
        // N s_29_2: jump b7
        return block_7(state, tracer, fn_state);
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
        // D s_30_3: not s_30_2
        let s_30_3: bool = !s_30_2;
        // D s_30_4: write-var gs#104663 <= s_30_3
        fn_state.gs_104663 = s_30_3;
        // N s_30_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call EL2Enabled(s_31_0)
        let s_31_1: bool = EL2Enabled(state, tracer, s_31_0);
        // D s_31_2: write-var gs#104662 <= s_31_1
        fn_state.gs_104662 = s_31_1;
        // N s_31_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_32_0: panic
        panic!("{:?}", ());
        // N s_32_1: return
        return;
    }
}
