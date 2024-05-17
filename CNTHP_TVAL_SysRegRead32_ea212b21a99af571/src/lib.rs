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
use u__UNKNOWN_bits::*;
use CNTHP_CTL_read::*;
use R_set::*;
use u_get_CNTHP_CTL_Type_ENABLE::*;
use u_get_SCR_Type_NS::*;
use CNTHP_CVAL_read::*;
use PhysicalCountInt::*;
use common::*;
pub fn CNTHP_TVAL_SysRegRead32_ea212b21a99af571<T: Tracer>(
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
        ga_166567: ProductType5c790c8ef59cc8b2,
        u__CNTHP_CTL_ENABLE: bool,
        ga_166556: ProductType5c790c8ef59cc8b2,
        u__PSTATE_EL: u8,
        u__SCR_NS: bool,
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
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call CNTHP_CTL_read(s_0_3)
        let s_0_4: ProductType700c18a878c5601b = CNTHP_CTL_read(state, tracer, s_0_3);
        // S s_0_5: call _get_CNTHP_CTL_Type_ENABLE(s_0_4)
        let s_0_5: bool = u_get_CNTHP_CTL_Type_ENABLE(state, tracer, s_0_4);
        // D s_0_6: write-var __CNTHP_CTL_ENABLE <= s_0_5
        fn_state.u__CNTHP_CTL_ENABLE = s_0_5;
        // C s_0_7: const #20920u : u32
        let s_0_7: u32 = 20920;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_SCR_Type_NS(s_0_8)
        let s_0_9: bool = u_get_SCR_Type_NS(state, tracer, s_0_8);
        // D s_0_10: write-var __SCR_NS <= s_0_9
        fn_state.u__SCR_NS = s_0_9;
        // D s_0_11: read-var __PSTATE_EL:u8
        let s_0_11: u8 = fn_state.u__PSTATE_EL;
        // D s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 2u16);
        // C s_0_13: const #448u : u32
        let s_0_13: u32 = 448;
        // D s_0_14: read-reg s_0_13:u8
        let s_0_14: u8 = {
            let value = state.read_register::<u8>(s_0_13 as isize);
            tracer.read_register(s_0_13 as isize, value);
            value
        };
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 2u16);
        // D s_0_16: cmp-eq s_0_12 s_0_15
        let s_0_16: bool = ((s_0_12) == (s_0_15));
        // N s_0_17: branch s_0_16 b14 b1
        if s_0_16 {
            return block_14(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b13 b2
        if s_1_5 {
            return block_13(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b10 b3
        if s_2_5 {
            return block_10(state, tracer, fn_state);
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
        // D s_5_0: read-var __SCR_NS:u8
        let s_5_0: bool = fn_state.u__SCR_NS;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #0u : u8
        let s_5_2: bool = false;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b9 b6
        if s_5_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var __CNTHP_CTL_ENABLE:u8
        let s_6_0: bool = fn_state.u__CNTHP_CTL_ENABLE;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // C s_6_2: const #0u : u8
        let s_6_2: bool = false;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // N s_6_5: branch s_6_4 b8 b7
        if s_6_4 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call CNTHP_CVAL_read(s_7_0)
        let s_7_1: ProductType5c790c8ef59cc8b2 = CNTHP_CVAL_read(state, tracer, s_7_0);
        // D s_7_2: write-var ga#166567 <= s_7_1
        fn_state.ga_166567 = s_7_1;
        // D s_7_3: read-var ga#166567.0:struct
        let s_7_3: u64 = fn_state.ga_166567._0;
        // C s_7_4: const #() : ()
        let s_7_4: () = ();
        // S s_7_5: call PhysicalCountInt(s_7_4)
        let s_7_5: u64 = PhysicalCountInt(state, tracer, s_7_4);
        // D s_7_6: cast zx s_7_3 -> bv
        let s_7_6: Bits = Bits::new(s_7_3 as u128, 64u16);
        // S s_7_7: cast zx s_7_5 -> bv
        let s_7_7: Bits = Bits::new(s_7_5 as u128, 64u16);
        // D s_7_8: sub s_7_6 s_7_7
        let s_7_8: Bits = ((s_7_6) - (s_7_7));
        // D s_7_9: cast reint s_7_8 -> u64
        let s_7_9: u64 = (s_7_8.value() as u64);
        // C s_7_10: const #0s : i
        let s_7_10: i128 = 0;
        // D s_7_11: cast zx s_7_9 -> bv
        let s_7_11: Bits = Bits::new(s_7_9 as u128, 64u16);
        // C s_7_12: const #1s : i64
        let s_7_12: i64 = 1;
        // C s_7_13: cast zx s_7_12 -> i
        let s_7_13: i128 = (i128::try_from(s_7_12).unwrap());
        // C s_7_14: const #31s : i
        let s_7_14: i128 = 31;
        // C s_7_15: add s_7_14 s_7_13
        let s_7_15: i128 = (s_7_14 + s_7_13);
        // D s_7_16: bit-extract s_7_11 s_7_10 s_7_15
        let s_7_16: Bits = (Bits::new(
            ((s_7_11) >> (s_7_10)).value(),
            u16::try_from(s_7_15).unwrap(),
        ));
        // D s_7_17: cast reint s_7_16 -> u32
        let s_7_17: u32 = (s_7_16.value() as u32);
        // D s_7_18: read-var t:i
        let s_7_18: i128 = fn_state.t;
        // D s_7_19: call R_set(s_7_18, s_7_17)
        let s_7_19: () = R_set(state, tracer, s_7_18, s_7_17);
        // N s_7_20: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #32s : i64
        let s_8_0: i64 = 32;
        // C s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // S s_8_2: call __UNKNOWN_bits(s_8_1)
        let s_8_2: Bits = u__UNKNOWN_bits(state, tracer, s_8_1);
        // S s_8_3: cast reint s_8_2 -> u32
        let s_8_3: u32 = (s_8_2.value() as u32);
        // D s_8_4: read-var t:i
        let s_8_4: i128 = fn_state.t;
        // D s_8_5: call R_set(s_8_4, s_8_3)
        let s_8_5: () = R_set(state, tracer, s_8_4, s_8_3);
        // N s_8_6: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var __CNTHP_CTL_ENABLE:u8
        let s_10_0: bool = fn_state.u__CNTHP_CTL_ENABLE;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // C s_10_2: const #0u : u8
        let s_10_2: bool = false;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // N s_10_5: branch s_10_4 b12 b11
        if s_10_4 {
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
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call CNTHP_CVAL_read(s_11_0)
        let s_11_1: ProductType5c790c8ef59cc8b2 = CNTHP_CVAL_read(state, tracer, s_11_0);
        // D s_11_2: write-var ga#166556 <= s_11_1
        fn_state.ga_166556 = s_11_1;
        // D s_11_3: read-var ga#166556.0:struct
        let s_11_3: u64 = fn_state.ga_166556._0;
        // C s_11_4: const #() : ()
        let s_11_4: () = ();
        // S s_11_5: call PhysicalCountInt(s_11_4)
        let s_11_5: u64 = PhysicalCountInt(state, tracer, s_11_4);
        // D s_11_6: cast zx s_11_3 -> bv
        let s_11_6: Bits = Bits::new(s_11_3 as u128, 64u16);
        // S s_11_7: cast zx s_11_5 -> bv
        let s_11_7: Bits = Bits::new(s_11_5 as u128, 64u16);
        // D s_11_8: sub s_11_6 s_11_7
        let s_11_8: Bits = ((s_11_6) - (s_11_7));
        // D s_11_9: cast reint s_11_8 -> u64
        let s_11_9: u64 = (s_11_8.value() as u64);
        // C s_11_10: const #0s : i
        let s_11_10: i128 = 0;
        // D s_11_11: cast zx s_11_9 -> bv
        let s_11_11: Bits = Bits::new(s_11_9 as u128, 64u16);
        // C s_11_12: const #1s : i64
        let s_11_12: i64 = 1;
        // C s_11_13: cast zx s_11_12 -> i
        let s_11_13: i128 = (i128::try_from(s_11_12).unwrap());
        // C s_11_14: const #31s : i
        let s_11_14: i128 = 31;
        // C s_11_15: add s_11_14 s_11_13
        let s_11_15: i128 = (s_11_14 + s_11_13);
        // D s_11_16: bit-extract s_11_11 s_11_10 s_11_15
        let s_11_16: Bits = (Bits::new(
            ((s_11_11) >> (s_11_10)).value(),
            u16::try_from(s_11_15).unwrap(),
        ));
        // D s_11_17: cast reint s_11_16 -> u32
        let s_11_17: u32 = (s_11_16.value() as u32);
        // D s_11_18: read-var t:i
        let s_11_18: i128 = fn_state.t;
        // D s_11_19: call R_set(s_11_18, s_11_17)
        let s_11_19: () = R_set(state, tracer, s_11_18, s_11_17);
        // N s_11_20: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #32s : i64
        let s_12_0: i64 = 32;
        // C s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // S s_12_2: call __UNKNOWN_bits(s_12_1)
        let s_12_2: Bits = u__UNKNOWN_bits(state, tracer, s_12_1);
        // S s_12_3: cast reint s_12_2 -> u32
        let s_12_3: u32 = (s_12_2.value() as u32);
        // D s_12_4: read-var t:i
        let s_12_4: i128 = fn_state.t;
        // D s_12_5: call R_set(s_12_4, s_12_3)
        let s_12_5: () = R_set(state, tracer, s_12_4, s_12_3);
        // N s_12_6: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: panic
        panic!("{:?}", ());
        // N s_13_1: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: panic
        panic!("{:?}", ());
        // N s_14_1: return
        return;
    }
}
