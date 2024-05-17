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
use X_set::*;
use Zeros::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_NV::*;
use common::*;
pub fn CurrentEL_SysRegRead_94be544516d41cc8<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    op0: u8,
    op1: u8,
    CRn: u8,
    op2: u8,
    CRm: u8,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_58273: bool,
        u__HCR_EL2_NV: bool,
        el: u8,
        op0: u8,
        op1: u8,
        CRn: u8,
        op2: u8,
        CRm: u8,
        t: i128,
    }
    let fn_state = FunctionState {
        el,
        op0,
        op1,
        CRn,
        op2,
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
        // C s_0_0: const #102552u : u32
        let s_0_0: u32 = 102552;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_HCR_EL2_Type_NV(s_0_1)
        let s_0_2: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_0_1);
        // D s_0_3: write-var __HCR_EL2_NV <= s_0_2
        fn_state.u__HCR_EL2_NV = s_0_2;
        // C s_0_4: const #16975u : u32
        let s_0_4: u32 = 16975;
        // D s_0_5: read-reg s_0_4:u8
        let s_0_5: u8 = {
            let value = state.read_register::<u8>(s_0_4 as isize);
            tracer.read_register(s_0_4 as isize, value);
            value
        };
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 2u16);
        // C s_0_7: const #448u : u32
        let s_0_7: u32 = 448;
        // D s_0_8: read-reg s_0_7:u8
        let s_0_8: u8 = {
            let value = state.read_register::<u8>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 2u16);
        // D s_0_10: cmp-eq s_0_6 s_0_9
        let s_0_10: bool = ((s_0_6) == (s_0_9));
        // N s_0_11: branch s_0_10 b13 b1
        if s_0_10 {
            return block_13(state, tracer, fn_state);
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
        // N s_1_7: branch s_1_6 b7 b2
        if s_1_6 {
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
        // C s_2_0: const #16975u : u32
        let s_2_0: u32 = 16975;
        // D s_2_1: read-reg s_2_0:u8
        let s_2_1: u8 = {
            let value = state.read_register::<u8>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 2u16);
        // C s_2_3: const #432u : u32
        let s_2_3: u32 = 432;
        // D s_2_4: read-reg s_2_3:u8
        let s_2_4: u8 = {
            let value = state.read_register::<u8>(s_2_3 as isize);
            tracer.read_register(s_2_3 as isize, value);
            value
        };
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 2u16);
        // D s_2_6: cmp-eq s_2_2 s_2_5
        let s_2_6: bool = ((s_2_2) == (s_2_5));
        // N s_2_7: branch s_2_6 b6 b3
        if s_2_6 {
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
        // C s_3_0: const #16975u : u32
        let s_3_0: u32 = 16975;
        // D s_3_1: read-reg s_3_0:u8
        let s_3_1: u8 = {
            let value = state.read_register::<u8>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 2u16);
        // C s_3_3: const #424u : u32
        let s_3_3: u32 = 424;
        // D s_3_4: read-reg s_3_3:u8
        let s_3_4: u8 = {
            let value = state.read_register::<u8>(s_3_3 as isize);
            tracer.read_register(s_3_3 as isize, value);
            value
        };
        // D s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 2u16);
        // D s_3_6: cmp-eq s_3_2 s_3_5
        let s_3_6: bool = ((s_3_2) == (s_3_5));
        // N s_3_7: branch s_3_6 b5 b4
        if s_3_6 {
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
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // C s_5_1: const #60s : i
        let s_5_1: i128 = 60;
        // S s_5_2: call Zeros(s_5_1)
        let s_5_2: Bits = Zeros(state, tracer, s_5_1);
        // S s_5_3: cast reint s_5_2 -> u60
        let s_5_3: u64 = (s_5_2.value() as u64);
        // C s_5_4: const #16975u : u32
        let s_5_4: u32 = 16975;
        // D s_5_5: read-reg s_5_4:u8
        let s_5_5: u8 = {
            let value = state.read_register::<u8>(s_5_4 as isize);
            tracer.read_register(s_5_4 as isize, value);
            value
        };
        // S s_5_6: cast zx s_5_3 -> bv
        let s_5_6: Bits = Bits::new(s_5_3 as u128, 60u16);
        // D s_5_7: cast zx s_5_5 -> bv
        let s_5_7: Bits = Bits::new(s_5_5 as u128, 2u16);
        // S s_5_8: cast reint s_5_6 -> u128
        let s_5_8: u128 = (s_5_6.value() as u128);
        // D s_5_9: size-of s_5_6
        let s_5_9: u16 = s_5_6.length();
        // D s_5_10: cast reint s_5_7 -> u128
        let s_5_10: u128 = (s_5_7.value() as u128);
        // D s_5_11: size-of s_5_7
        let s_5_11: u16 = s_5_7.length();
        // D s_5_12: lsl s_5_8 s_5_11
        let s_5_12: u128 = s_5_8 << s_5_11;
        // D s_5_13: or s_5_12 s_5_10
        let s_5_13: u128 = ((s_5_12) | (s_5_10));
        // D s_5_14: add s_5_9 s_5_11
        let s_5_14: u16 = (s_5_9 + s_5_11);
        // D s_5_15: create-bits s_5_13 s_5_14
        let s_5_15: Bits = Bits::new(s_5_13, s_5_14);
        // D s_5_16: cast reint s_5_15 -> u62
        let s_5_16: u64 = (s_5_15.value() as u64);
        // C s_5_17: const #2s : i
        let s_5_17: i128 = 2;
        // S s_5_18: call Zeros(s_5_17)
        let s_5_18: Bits = Zeros(state, tracer, s_5_17);
        // S s_5_19: cast reint s_5_18 -> u8
        let s_5_19: u8 = (s_5_18.value() as u8);
        // D s_5_20: cast zx s_5_16 -> bv
        let s_5_20: Bits = Bits::new(s_5_16 as u128, 62u16);
        // S s_5_21: cast zx s_5_19 -> bv
        let s_5_21: Bits = Bits::new(s_5_19 as u128, 2u16);
        // D s_5_22: cast reint s_5_20 -> u128
        let s_5_22: u128 = (s_5_20.value() as u128);
        // D s_5_23: size-of s_5_20
        let s_5_23: u16 = s_5_20.length();
        // S s_5_24: cast reint s_5_21 -> u128
        let s_5_24: u128 = (s_5_21.value() as u128);
        // D s_5_25: size-of s_5_21
        let s_5_25: u16 = s_5_21.length();
        // D s_5_26: lsl s_5_22 s_5_25
        let s_5_26: u128 = s_5_22 << s_5_25;
        // D s_5_27: or s_5_26 s_5_24
        let s_5_27: u128 = ((s_5_26) | (s_5_24));
        // D s_5_28: add s_5_23 s_5_25
        let s_5_28: u16 = (s_5_23 + s_5_25);
        // D s_5_29: create-bits s_5_27 s_5_28
        let s_5_29: Bits = Bits::new(s_5_27, s_5_28);
        // D s_5_30: cast reint s_5_29 -> u64
        let s_5_30: u64 = (s_5_29.value() as u64);
        // D s_5_31: cast zx s_5_30 -> bv
        let s_5_31: Bits = Bits::new(s_5_30 as u128, 64u16);
        // D s_5_32: read-var t:i
        let s_5_32: i128 = fn_state.t;
        // D s_5_33: call X_set(s_5_32, s_5_0, s_5_31)
        let s_5_33: () = X_set(state, tracer, s_5_32, s_5_0, s_5_31);
        // N s_5_34: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // C s_6_1: const #60s : i
        let s_6_1: i128 = 60;
        // S s_6_2: call Zeros(s_6_1)
        let s_6_2: Bits = Zeros(state, tracer, s_6_1);
        // S s_6_3: cast reint s_6_2 -> u60
        let s_6_3: u64 = (s_6_2.value() as u64);
        // C s_6_4: const #16975u : u32
        let s_6_4: u32 = 16975;
        // D s_6_5: read-reg s_6_4:u8
        let s_6_5: u8 = {
            let value = state.read_register::<u8>(s_6_4 as isize);
            tracer.read_register(s_6_4 as isize, value);
            value
        };
        // S s_6_6: cast zx s_6_3 -> bv
        let s_6_6: Bits = Bits::new(s_6_3 as u128, 60u16);
        // D s_6_7: cast zx s_6_5 -> bv
        let s_6_7: Bits = Bits::new(s_6_5 as u128, 2u16);
        // S s_6_8: cast reint s_6_6 -> u128
        let s_6_8: u128 = (s_6_6.value() as u128);
        // D s_6_9: size-of s_6_6
        let s_6_9: u16 = s_6_6.length();
        // D s_6_10: cast reint s_6_7 -> u128
        let s_6_10: u128 = (s_6_7.value() as u128);
        // D s_6_11: size-of s_6_7
        let s_6_11: u16 = s_6_7.length();
        // D s_6_12: lsl s_6_8 s_6_11
        let s_6_12: u128 = s_6_8 << s_6_11;
        // D s_6_13: or s_6_12 s_6_10
        let s_6_13: u128 = ((s_6_12) | (s_6_10));
        // D s_6_14: add s_6_9 s_6_11
        let s_6_14: u16 = (s_6_9 + s_6_11);
        // D s_6_15: create-bits s_6_13 s_6_14
        let s_6_15: Bits = Bits::new(s_6_13, s_6_14);
        // D s_6_16: cast reint s_6_15 -> u62
        let s_6_16: u64 = (s_6_15.value() as u64);
        // C s_6_17: const #2s : i
        let s_6_17: i128 = 2;
        // S s_6_18: call Zeros(s_6_17)
        let s_6_18: Bits = Zeros(state, tracer, s_6_17);
        // S s_6_19: cast reint s_6_18 -> u8
        let s_6_19: u8 = (s_6_18.value() as u8);
        // D s_6_20: cast zx s_6_16 -> bv
        let s_6_20: Bits = Bits::new(s_6_16 as u128, 62u16);
        // S s_6_21: cast zx s_6_19 -> bv
        let s_6_21: Bits = Bits::new(s_6_19 as u128, 2u16);
        // D s_6_22: cast reint s_6_20 -> u128
        let s_6_22: u128 = (s_6_20.value() as u128);
        // D s_6_23: size-of s_6_20
        let s_6_23: u16 = s_6_20.length();
        // S s_6_24: cast reint s_6_21 -> u128
        let s_6_24: u128 = (s_6_21.value() as u128);
        // D s_6_25: size-of s_6_21
        let s_6_25: u16 = s_6_21.length();
        // D s_6_26: lsl s_6_22 s_6_25
        let s_6_26: u128 = s_6_22 << s_6_25;
        // D s_6_27: or s_6_26 s_6_24
        let s_6_27: u128 = ((s_6_26) | (s_6_24));
        // D s_6_28: add s_6_23 s_6_25
        let s_6_28: u16 = (s_6_23 + s_6_25);
        // D s_6_29: create-bits s_6_27 s_6_28
        let s_6_29: Bits = Bits::new(s_6_27, s_6_28);
        // D s_6_30: cast reint s_6_29 -> u64
        let s_6_30: u64 = (s_6_29.value() as u64);
        // D s_6_31: cast zx s_6_30 -> bv
        let s_6_31: Bits = Bits::new(s_6_30 as u128, 64u16);
        // D s_6_32: read-var t:i
        let s_6_32: i128 = fn_state.t;
        // D s_6_33: call X_set(s_6_32, s_6_0, s_6_31)
        let s_6_33: () = X_set(state, tracer, s_6_32, s_6_0, s_6_31);
        // N s_6_34: return
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
        // N s_7_2: branch s_7_1 b12 b8
        if s_7_1 {
            return block_12(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#58273 <= s_8_0
        fn_state.gs_58273 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#58273:u8
        let s_9_0: bool = fn_state.gs_58273;
        // N s_9_1: branch s_9_0 b11 b10
        if s_9_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #64s : i64
        let s_10_0: i64 = 64;
        // C s_10_1: const #60s : i
        let s_10_1: i128 = 60;
        // S s_10_2: call Zeros(s_10_1)
        let s_10_2: Bits = Zeros(state, tracer, s_10_1);
        // S s_10_3: cast reint s_10_2 -> u60
        let s_10_3: u64 = (s_10_2.value() as u64);
        // C s_10_4: const #16975u : u32
        let s_10_4: u32 = 16975;
        // D s_10_5: read-reg s_10_4:u8
        let s_10_5: u8 = {
            let value = state.read_register::<u8>(s_10_4 as isize);
            tracer.read_register(s_10_4 as isize, value);
            value
        };
        // S s_10_6: cast zx s_10_3 -> bv
        let s_10_6: Bits = Bits::new(s_10_3 as u128, 60u16);
        // D s_10_7: cast zx s_10_5 -> bv
        let s_10_7: Bits = Bits::new(s_10_5 as u128, 2u16);
        // S s_10_8: cast reint s_10_6 -> u128
        let s_10_8: u128 = (s_10_6.value() as u128);
        // D s_10_9: size-of s_10_6
        let s_10_9: u16 = s_10_6.length();
        // D s_10_10: cast reint s_10_7 -> u128
        let s_10_10: u128 = (s_10_7.value() as u128);
        // D s_10_11: size-of s_10_7
        let s_10_11: u16 = s_10_7.length();
        // D s_10_12: lsl s_10_8 s_10_11
        let s_10_12: u128 = s_10_8 << s_10_11;
        // D s_10_13: or s_10_12 s_10_10
        let s_10_13: u128 = ((s_10_12) | (s_10_10));
        // D s_10_14: add s_10_9 s_10_11
        let s_10_14: u16 = (s_10_9 + s_10_11);
        // D s_10_15: create-bits s_10_13 s_10_14
        let s_10_15: Bits = Bits::new(s_10_13, s_10_14);
        // D s_10_16: cast reint s_10_15 -> u62
        let s_10_16: u64 = (s_10_15.value() as u64);
        // C s_10_17: const #2s : i
        let s_10_17: i128 = 2;
        // S s_10_18: call Zeros(s_10_17)
        let s_10_18: Bits = Zeros(state, tracer, s_10_17);
        // S s_10_19: cast reint s_10_18 -> u8
        let s_10_19: u8 = (s_10_18.value() as u8);
        // D s_10_20: cast zx s_10_16 -> bv
        let s_10_20: Bits = Bits::new(s_10_16 as u128, 62u16);
        // S s_10_21: cast zx s_10_19 -> bv
        let s_10_21: Bits = Bits::new(s_10_19 as u128, 2u16);
        // D s_10_22: cast reint s_10_20 -> u128
        let s_10_22: u128 = (s_10_20.value() as u128);
        // D s_10_23: size-of s_10_20
        let s_10_23: u16 = s_10_20.length();
        // S s_10_24: cast reint s_10_21 -> u128
        let s_10_24: u128 = (s_10_21.value() as u128);
        // D s_10_25: size-of s_10_21
        let s_10_25: u16 = s_10_21.length();
        // D s_10_26: lsl s_10_22 s_10_25
        let s_10_26: u128 = s_10_22 << s_10_25;
        // D s_10_27: or s_10_26 s_10_24
        let s_10_27: u128 = ((s_10_26) | (s_10_24));
        // D s_10_28: add s_10_23 s_10_25
        let s_10_28: u16 = (s_10_23 + s_10_25);
        // D s_10_29: create-bits s_10_27 s_10_28
        let s_10_29: Bits = Bits::new(s_10_27, s_10_28);
        // D s_10_30: cast reint s_10_29 -> u64
        let s_10_30: u64 = (s_10_29.value() as u64);
        // D s_10_31: cast zx s_10_30 -> bv
        let s_10_31: Bits = Bits::new(s_10_30 as u128, 64u16);
        // D s_10_32: read-var t:i
        let s_10_32: i128 = fn_state.t;
        // D s_10_33: call X_set(s_10_32, s_10_0, s_10_31)
        let s_10_33: () = X_set(state, tracer, s_10_32, s_10_0, s_10_31);
        // N s_10_34: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #64s : i64
        let s_11_0: i64 = 64;
        // C s_11_1: const #60s : i
        let s_11_1: i128 = 60;
        // S s_11_2: call Zeros(s_11_1)
        let s_11_2: Bits = Zeros(state, tracer, s_11_1);
        // S s_11_3: cast reint s_11_2 -> u60
        let s_11_3: u64 = (s_11_2.value() as u64);
        // S s_11_4: cast zx s_11_3 -> bv
        let s_11_4: Bits = Bits::new(s_11_3 as u128, 60u16);
        // C s_11_5: const #2u : u8
        let s_11_5: u8 = 2;
        // C s_11_6: cast zx s_11_5 -> bv
        let s_11_6: Bits = Bits::new(s_11_5 as u128, 2u16);
        // S s_11_7: cast reint s_11_4 -> u128
        let s_11_7: u128 = (s_11_4.value() as u128);
        // D s_11_8: size-of s_11_4
        let s_11_8: u16 = s_11_4.length();
        // C s_11_9: cast reint s_11_6 -> u128
        let s_11_9: u128 = (s_11_6.value() as u128);
        // D s_11_10: size-of s_11_6
        let s_11_10: u16 = s_11_6.length();
        // D s_11_11: lsl s_11_7 s_11_10
        let s_11_11: u128 = s_11_7 << s_11_10;
        // D s_11_12: or s_11_11 s_11_9
        let s_11_12: u128 = ((s_11_11) | (s_11_9));
        // D s_11_13: add s_11_8 s_11_10
        let s_11_13: u16 = (s_11_8 + s_11_10);
        // D s_11_14: create-bits s_11_12 s_11_13
        let s_11_14: Bits = Bits::new(s_11_12, s_11_13);
        // D s_11_15: cast reint s_11_14 -> u62
        let s_11_15: u64 = (s_11_14.value() as u64);
        // C s_11_16: const #2s : i
        let s_11_16: i128 = 2;
        // S s_11_17: call Zeros(s_11_16)
        let s_11_17: Bits = Zeros(state, tracer, s_11_16);
        // S s_11_18: cast reint s_11_17 -> u8
        let s_11_18: u8 = (s_11_17.value() as u8);
        // D s_11_19: cast zx s_11_15 -> bv
        let s_11_19: Bits = Bits::new(s_11_15 as u128, 62u16);
        // S s_11_20: cast zx s_11_18 -> bv
        let s_11_20: Bits = Bits::new(s_11_18 as u128, 2u16);
        // D s_11_21: cast reint s_11_19 -> u128
        let s_11_21: u128 = (s_11_19.value() as u128);
        // D s_11_22: size-of s_11_19
        let s_11_22: u16 = s_11_19.length();
        // S s_11_23: cast reint s_11_20 -> u128
        let s_11_23: u128 = (s_11_20.value() as u128);
        // D s_11_24: size-of s_11_20
        let s_11_24: u16 = s_11_20.length();
        // D s_11_25: lsl s_11_21 s_11_24
        let s_11_25: u128 = s_11_21 << s_11_24;
        // D s_11_26: or s_11_25 s_11_23
        let s_11_26: u128 = ((s_11_25) | (s_11_23));
        // D s_11_27: add s_11_22 s_11_24
        let s_11_27: u16 = (s_11_22 + s_11_24);
        // D s_11_28: create-bits s_11_26 s_11_27
        let s_11_28: Bits = Bits::new(s_11_26, s_11_27);
        // D s_11_29: cast reint s_11_28 -> u64
        let s_11_29: u64 = (s_11_28.value() as u64);
        // D s_11_30: cast zx s_11_29 -> bv
        let s_11_30: Bits = Bits::new(s_11_29 as u128, 64u16);
        // D s_11_31: read-var t:i
        let s_11_31: i128 = fn_state.t;
        // D s_11_32: call X_set(s_11_31, s_11_0, s_11_30)
        let s_11_32: () = X_set(state, tracer, s_11_31, s_11_0, s_11_30);
        // N s_11_33: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var __HCR_EL2_NV:u8
        let s_12_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 1u16);
        // C s_12_2: const #1u : u8
        let s_12_2: bool = true;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: write-var gs#58273 <= s_12_4
        fn_state.gs_58273 = s_12_4;
        // N s_12_6: jump b9
        return block_9(state, tracer, fn_state);
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
}
