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
use X_read::*;
use common::*;
pub fn NZCV_SysRegWrite_a24ec14709a57410<T: Tracer>(
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
        u__PSTATE_EL: u8,
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
        // D s_0_3: read-var __PSTATE_EL:u8
        let s_0_3: u8 = fn_state.u__PSTATE_EL;
        // D s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 2u16);
        // C s_0_5: const #448u : u32
        let s_0_5: u32 = 448;
        // D s_0_6: read-reg s_0_5:u8
        let s_0_6: u8 = {
            let value = state.read_register::<u8>(s_0_5 as isize);
            tracer.read_register(s_0_5 as isize, value);
            value
        };
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 2u16);
        // D s_0_8: cmp-eq s_0_4 s_0_7
        let s_0_8: bool = ((s_0_4) == (s_0_7));
        // N s_0_9: branch s_0_8 b8 b1
        if s_0_8 {
            return block_8(state, tracer, fn_state);
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
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // D s_5_1: read-var t:i
        let s_5_1: i128 = fn_state.t;
        // D s_5_2: call X_read(s_5_1, s_5_0)
        let s_5_2: Bits = X_read(state, tracer, s_5_1, s_5_0);
        // D s_5_3: cast reint s_5_2 -> u64
        let s_5_3: u64 = (s_5_2.value() as u64);
        // C s_5_4: const #28s : i
        let s_5_4: i128 = 28;
        // D s_5_5: cast zx s_5_3 -> bv
        let s_5_5: Bits = Bits::new(s_5_3 as u128, 64u16);
        // C s_5_6: const #1s : i64
        let s_5_6: i64 = 1;
        // C s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // C s_5_8: const #3s : i
        let s_5_8: i128 = 3;
        // C s_5_9: add s_5_8 s_5_7
        let s_5_9: i128 = (s_5_8 + s_5_7);
        // D s_5_10: bit-extract s_5_5 s_5_4 s_5_9
        let s_5_10: Bits = (Bits::new(
            ((s_5_5) >> (s_5_4)).value(),
            u16::try_from(s_5_9).unwrap(),
        ));
        // D s_5_11: cast reint s_5_10 -> u8
        let s_5_11: u8 = (s_5_10.value() as u8);
        // C s_5_12: const #3s : i
        let s_5_12: i128 = 3;
        // D s_5_13: cast zx s_5_11 -> bv
        let s_5_13: Bits = Bits::new(s_5_11 as u128, 4u16);
        // C s_5_14: const #1s : i64
        let s_5_14: i64 = 1;
        // C s_5_15: cast zx s_5_14 -> i
        let s_5_15: i128 = (i128::try_from(s_5_14).unwrap());
        // C s_5_16: const #0s : i
        let s_5_16: i128 = 0;
        // C s_5_17: add s_5_16 s_5_15
        let s_5_17: i128 = (s_5_16 + s_5_15);
        // D s_5_18: bit-extract s_5_13 s_5_12 s_5_17
        let s_5_18: Bits = (Bits::new(
            ((s_5_13) >> (s_5_12)).value(),
            u16::try_from(s_5_17).unwrap(),
        ));
        // D s_5_19: cast reint s_5_18 -> u8
        let s_5_19: bool = ((s_5_18.value()) != 0);
        // C s_5_20: const #16984u : u32
        let s_5_20: u32 = 16984;
        // N s_5_21: write-reg s_5_20 <= s_5_19
        let s_5_21: () = {
            state.write_register::<bool>(s_5_20 as isize, s_5_19);
            tracer.write_register(s_5_20 as isize, s_5_19);
        };
        // C s_5_22: const #2s : i
        let s_5_22: i128 = 2;
        // D s_5_23: cast zx s_5_11 -> bv
        let s_5_23: Bits = Bits::new(s_5_11 as u128, 4u16);
        // C s_5_24: const #1s : i64
        let s_5_24: i64 = 1;
        // C s_5_25: cast zx s_5_24 -> i
        let s_5_25: i128 = (i128::try_from(s_5_24).unwrap());
        // C s_5_26: const #0s : i
        let s_5_26: i128 = 0;
        // C s_5_27: add s_5_26 s_5_25
        let s_5_27: i128 = (s_5_26 + s_5_25);
        // D s_5_28: bit-extract s_5_23 s_5_22 s_5_27
        let s_5_28: Bits = (Bits::new(
            ((s_5_23) >> (s_5_22)).value(),
            u16::try_from(s_5_27).unwrap(),
        ));
        // D s_5_29: cast reint s_5_28 -> u8
        let s_5_29: bool = ((s_5_28.value()) != 0);
        // C s_5_30: const #16997u : u32
        let s_5_30: u32 = 16997;
        // N s_5_31: write-reg s_5_30 <= s_5_29
        let s_5_31: () = {
            state.write_register::<bool>(s_5_30 as isize, s_5_29);
            tracer.write_register(s_5_30 as isize, s_5_29);
        };
        // C s_5_32: const #1s : i
        let s_5_32: i128 = 1;
        // D s_5_33: cast zx s_5_11 -> bv
        let s_5_33: Bits = Bits::new(s_5_11 as u128, 4u16);
        // C s_5_34: const #1s : i64
        let s_5_34: i64 = 1;
        // C s_5_35: cast zx s_5_34 -> i
        let s_5_35: i128 = (i128::try_from(s_5_34).unwrap());
        // C s_5_36: const #0s : i
        let s_5_36: i128 = 0;
        // C s_5_37: add s_5_36 s_5_35
        let s_5_37: i128 = (s_5_36 + s_5_35);
        // D s_5_38: bit-extract s_5_33 s_5_32 s_5_37
        let s_5_38: Bits = (Bits::new(
            ((s_5_33) >> (s_5_32)).value(),
            u16::try_from(s_5_37).unwrap(),
        ));
        // D s_5_39: cast reint s_5_38 -> u8
        let s_5_39: bool = ((s_5_38.value()) != 0);
        // C s_5_40: const #16971u : u32
        let s_5_40: u32 = 16971;
        // N s_5_41: write-reg s_5_40 <= s_5_39
        let s_5_41: () = {
            state.write_register::<bool>(s_5_40 as isize, s_5_39);
            tracer.write_register(s_5_40 as isize, s_5_39);
        };
        // C s_5_42: const #0s : i
        let s_5_42: i128 = 0;
        // D s_5_43: cast zx s_5_11 -> bv
        let s_5_43: Bits = Bits::new(s_5_11 as u128, 4u16);
        // C s_5_44: const #1s : i64
        let s_5_44: i64 = 1;
        // C s_5_45: cast zx s_5_44 -> i
        let s_5_45: i128 = (i128::try_from(s_5_44).unwrap());
        // C s_5_46: const #0s : i
        let s_5_46: i128 = 0;
        // C s_5_47: add s_5_46 s_5_45
        let s_5_47: i128 = (s_5_46 + s_5_45);
        // D s_5_48: bit-extract s_5_43 s_5_42 s_5_47
        let s_5_48: Bits = (Bits::new(
            ((s_5_43) >> (s_5_42)).value(),
            u16::try_from(s_5_47).unwrap(),
        ));
        // D s_5_49: cast reint s_5_48 -> u8
        let s_5_49: bool = ((s_5_48.value()) != 0);
        // C s_5_50: const #16996u : u32
        let s_5_50: u32 = 16996;
        // N s_5_51: write-reg s_5_50 <= s_5_49
        let s_5_51: () = {
            state.write_register::<bool>(s_5_50 as isize, s_5_49);
            tracer.write_register(s_5_50 as isize, s_5_49);
        };
        // N s_5_52: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // D s_6_1: read-var t:i
        let s_6_1: i128 = fn_state.t;
        // D s_6_2: call X_read(s_6_1, s_6_0)
        let s_6_2: Bits = X_read(state, tracer, s_6_1, s_6_0);
        // D s_6_3: cast reint s_6_2 -> u64
        let s_6_3: u64 = (s_6_2.value() as u64);
        // C s_6_4: const #28s : i
        let s_6_4: i128 = 28;
        // D s_6_5: cast zx s_6_3 -> bv
        let s_6_5: Bits = Bits::new(s_6_3 as u128, 64u16);
        // C s_6_6: const #1s : i64
        let s_6_6: i64 = 1;
        // C s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (i128::try_from(s_6_6).unwrap());
        // C s_6_8: const #3s : i
        let s_6_8: i128 = 3;
        // C s_6_9: add s_6_8 s_6_7
        let s_6_9: i128 = (s_6_8 + s_6_7);
        // D s_6_10: bit-extract s_6_5 s_6_4 s_6_9
        let s_6_10: Bits = (Bits::new(
            ((s_6_5) >> (s_6_4)).value(),
            u16::try_from(s_6_9).unwrap(),
        ));
        // D s_6_11: cast reint s_6_10 -> u8
        let s_6_11: u8 = (s_6_10.value() as u8);
        // C s_6_12: const #3s : i
        let s_6_12: i128 = 3;
        // D s_6_13: cast zx s_6_11 -> bv
        let s_6_13: Bits = Bits::new(s_6_11 as u128, 4u16);
        // C s_6_14: const #1s : i64
        let s_6_14: i64 = 1;
        // C s_6_15: cast zx s_6_14 -> i
        let s_6_15: i128 = (i128::try_from(s_6_14).unwrap());
        // C s_6_16: const #0s : i
        let s_6_16: i128 = 0;
        // C s_6_17: add s_6_16 s_6_15
        let s_6_17: i128 = (s_6_16 + s_6_15);
        // D s_6_18: bit-extract s_6_13 s_6_12 s_6_17
        let s_6_18: Bits = (Bits::new(
            ((s_6_13) >> (s_6_12)).value(),
            u16::try_from(s_6_17).unwrap(),
        ));
        // D s_6_19: cast reint s_6_18 -> u8
        let s_6_19: bool = ((s_6_18.value()) != 0);
        // C s_6_20: const #16984u : u32
        let s_6_20: u32 = 16984;
        // N s_6_21: write-reg s_6_20 <= s_6_19
        let s_6_21: () = {
            state.write_register::<bool>(s_6_20 as isize, s_6_19);
            tracer.write_register(s_6_20 as isize, s_6_19);
        };
        // C s_6_22: const #2s : i
        let s_6_22: i128 = 2;
        // D s_6_23: cast zx s_6_11 -> bv
        let s_6_23: Bits = Bits::new(s_6_11 as u128, 4u16);
        // C s_6_24: const #1s : i64
        let s_6_24: i64 = 1;
        // C s_6_25: cast zx s_6_24 -> i
        let s_6_25: i128 = (i128::try_from(s_6_24).unwrap());
        // C s_6_26: const #0s : i
        let s_6_26: i128 = 0;
        // C s_6_27: add s_6_26 s_6_25
        let s_6_27: i128 = (s_6_26 + s_6_25);
        // D s_6_28: bit-extract s_6_23 s_6_22 s_6_27
        let s_6_28: Bits = (Bits::new(
            ((s_6_23) >> (s_6_22)).value(),
            u16::try_from(s_6_27).unwrap(),
        ));
        // D s_6_29: cast reint s_6_28 -> u8
        let s_6_29: bool = ((s_6_28.value()) != 0);
        // C s_6_30: const #16997u : u32
        let s_6_30: u32 = 16997;
        // N s_6_31: write-reg s_6_30 <= s_6_29
        let s_6_31: () = {
            state.write_register::<bool>(s_6_30 as isize, s_6_29);
            tracer.write_register(s_6_30 as isize, s_6_29);
        };
        // C s_6_32: const #1s : i
        let s_6_32: i128 = 1;
        // D s_6_33: cast zx s_6_11 -> bv
        let s_6_33: Bits = Bits::new(s_6_11 as u128, 4u16);
        // C s_6_34: const #1s : i64
        let s_6_34: i64 = 1;
        // C s_6_35: cast zx s_6_34 -> i
        let s_6_35: i128 = (i128::try_from(s_6_34).unwrap());
        // C s_6_36: const #0s : i
        let s_6_36: i128 = 0;
        // C s_6_37: add s_6_36 s_6_35
        let s_6_37: i128 = (s_6_36 + s_6_35);
        // D s_6_38: bit-extract s_6_33 s_6_32 s_6_37
        let s_6_38: Bits = (Bits::new(
            ((s_6_33) >> (s_6_32)).value(),
            u16::try_from(s_6_37).unwrap(),
        ));
        // D s_6_39: cast reint s_6_38 -> u8
        let s_6_39: bool = ((s_6_38.value()) != 0);
        // C s_6_40: const #16971u : u32
        let s_6_40: u32 = 16971;
        // N s_6_41: write-reg s_6_40 <= s_6_39
        let s_6_41: () = {
            state.write_register::<bool>(s_6_40 as isize, s_6_39);
            tracer.write_register(s_6_40 as isize, s_6_39);
        };
        // C s_6_42: const #0s : i
        let s_6_42: i128 = 0;
        // D s_6_43: cast zx s_6_11 -> bv
        let s_6_43: Bits = Bits::new(s_6_11 as u128, 4u16);
        // C s_6_44: const #1s : i64
        let s_6_44: i64 = 1;
        // C s_6_45: cast zx s_6_44 -> i
        let s_6_45: i128 = (i128::try_from(s_6_44).unwrap());
        // C s_6_46: const #0s : i
        let s_6_46: i128 = 0;
        // C s_6_47: add s_6_46 s_6_45
        let s_6_47: i128 = (s_6_46 + s_6_45);
        // D s_6_48: bit-extract s_6_43 s_6_42 s_6_47
        let s_6_48: Bits = (Bits::new(
            ((s_6_43) >> (s_6_42)).value(),
            u16::try_from(s_6_47).unwrap(),
        ));
        // D s_6_49: cast reint s_6_48 -> u8
        let s_6_49: bool = ((s_6_48.value()) != 0);
        // C s_6_50: const #16996u : u32
        let s_6_50: u32 = 16996;
        // N s_6_51: write-reg s_6_50 <= s_6_49
        let s_6_51: () = {
            state.write_register::<bool>(s_6_50 as isize, s_6_49);
            tracer.write_register(s_6_50 as isize, s_6_49);
        };
        // N s_6_52: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // D s_7_1: read-var t:i
        let s_7_1: i128 = fn_state.t;
        // D s_7_2: call X_read(s_7_1, s_7_0)
        let s_7_2: Bits = X_read(state, tracer, s_7_1, s_7_0);
        // D s_7_3: cast reint s_7_2 -> u64
        let s_7_3: u64 = (s_7_2.value() as u64);
        // C s_7_4: const #28s : i
        let s_7_4: i128 = 28;
        // D s_7_5: cast zx s_7_3 -> bv
        let s_7_5: Bits = Bits::new(s_7_3 as u128, 64u16);
        // C s_7_6: const #1s : i64
        let s_7_6: i64 = 1;
        // C s_7_7: cast zx s_7_6 -> i
        let s_7_7: i128 = (i128::try_from(s_7_6).unwrap());
        // C s_7_8: const #3s : i
        let s_7_8: i128 = 3;
        // C s_7_9: add s_7_8 s_7_7
        let s_7_9: i128 = (s_7_8 + s_7_7);
        // D s_7_10: bit-extract s_7_5 s_7_4 s_7_9
        let s_7_10: Bits = (Bits::new(
            ((s_7_5) >> (s_7_4)).value(),
            u16::try_from(s_7_9).unwrap(),
        ));
        // D s_7_11: cast reint s_7_10 -> u8
        let s_7_11: u8 = (s_7_10.value() as u8);
        // C s_7_12: const #3s : i
        let s_7_12: i128 = 3;
        // D s_7_13: cast zx s_7_11 -> bv
        let s_7_13: Bits = Bits::new(s_7_11 as u128, 4u16);
        // C s_7_14: const #1s : i64
        let s_7_14: i64 = 1;
        // C s_7_15: cast zx s_7_14 -> i
        let s_7_15: i128 = (i128::try_from(s_7_14).unwrap());
        // C s_7_16: const #0s : i
        let s_7_16: i128 = 0;
        // C s_7_17: add s_7_16 s_7_15
        let s_7_17: i128 = (s_7_16 + s_7_15);
        // D s_7_18: bit-extract s_7_13 s_7_12 s_7_17
        let s_7_18: Bits = (Bits::new(
            ((s_7_13) >> (s_7_12)).value(),
            u16::try_from(s_7_17).unwrap(),
        ));
        // D s_7_19: cast reint s_7_18 -> u8
        let s_7_19: bool = ((s_7_18.value()) != 0);
        // C s_7_20: const #16984u : u32
        let s_7_20: u32 = 16984;
        // N s_7_21: write-reg s_7_20 <= s_7_19
        let s_7_21: () = {
            state.write_register::<bool>(s_7_20 as isize, s_7_19);
            tracer.write_register(s_7_20 as isize, s_7_19);
        };
        // C s_7_22: const #2s : i
        let s_7_22: i128 = 2;
        // D s_7_23: cast zx s_7_11 -> bv
        let s_7_23: Bits = Bits::new(s_7_11 as u128, 4u16);
        // C s_7_24: const #1s : i64
        let s_7_24: i64 = 1;
        // C s_7_25: cast zx s_7_24 -> i
        let s_7_25: i128 = (i128::try_from(s_7_24).unwrap());
        // C s_7_26: const #0s : i
        let s_7_26: i128 = 0;
        // C s_7_27: add s_7_26 s_7_25
        let s_7_27: i128 = (s_7_26 + s_7_25);
        // D s_7_28: bit-extract s_7_23 s_7_22 s_7_27
        let s_7_28: Bits = (Bits::new(
            ((s_7_23) >> (s_7_22)).value(),
            u16::try_from(s_7_27).unwrap(),
        ));
        // D s_7_29: cast reint s_7_28 -> u8
        let s_7_29: bool = ((s_7_28.value()) != 0);
        // C s_7_30: const #16997u : u32
        let s_7_30: u32 = 16997;
        // N s_7_31: write-reg s_7_30 <= s_7_29
        let s_7_31: () = {
            state.write_register::<bool>(s_7_30 as isize, s_7_29);
            tracer.write_register(s_7_30 as isize, s_7_29);
        };
        // C s_7_32: const #1s : i
        let s_7_32: i128 = 1;
        // D s_7_33: cast zx s_7_11 -> bv
        let s_7_33: Bits = Bits::new(s_7_11 as u128, 4u16);
        // C s_7_34: const #1s : i64
        let s_7_34: i64 = 1;
        // C s_7_35: cast zx s_7_34 -> i
        let s_7_35: i128 = (i128::try_from(s_7_34).unwrap());
        // C s_7_36: const #0s : i
        let s_7_36: i128 = 0;
        // C s_7_37: add s_7_36 s_7_35
        let s_7_37: i128 = (s_7_36 + s_7_35);
        // D s_7_38: bit-extract s_7_33 s_7_32 s_7_37
        let s_7_38: Bits = (Bits::new(
            ((s_7_33) >> (s_7_32)).value(),
            u16::try_from(s_7_37).unwrap(),
        ));
        // D s_7_39: cast reint s_7_38 -> u8
        let s_7_39: bool = ((s_7_38.value()) != 0);
        // C s_7_40: const #16971u : u32
        let s_7_40: u32 = 16971;
        // N s_7_41: write-reg s_7_40 <= s_7_39
        let s_7_41: () = {
            state.write_register::<bool>(s_7_40 as isize, s_7_39);
            tracer.write_register(s_7_40 as isize, s_7_39);
        };
        // C s_7_42: const #0s : i
        let s_7_42: i128 = 0;
        // D s_7_43: cast zx s_7_11 -> bv
        let s_7_43: Bits = Bits::new(s_7_11 as u128, 4u16);
        // C s_7_44: const #1s : i64
        let s_7_44: i64 = 1;
        // C s_7_45: cast zx s_7_44 -> i
        let s_7_45: i128 = (i128::try_from(s_7_44).unwrap());
        // C s_7_46: const #0s : i
        let s_7_46: i128 = 0;
        // C s_7_47: add s_7_46 s_7_45
        let s_7_47: i128 = (s_7_46 + s_7_45);
        // D s_7_48: bit-extract s_7_43 s_7_42 s_7_47
        let s_7_48: Bits = (Bits::new(
            ((s_7_43) >> (s_7_42)).value(),
            u16::try_from(s_7_47).unwrap(),
        ));
        // D s_7_49: cast reint s_7_48 -> u8
        let s_7_49: bool = ((s_7_48.value()) != 0);
        // C s_7_50: const #16996u : u32
        let s_7_50: u32 = 16996;
        // N s_7_51: write-reg s_7_50 <= s_7_49
        let s_7_51: () = {
            state.write_register::<bool>(s_7_50 as isize, s_7_49);
            tracer.write_register(s_7_50 as isize, s_7_49);
        };
        // N s_7_52: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #64s : i64
        let s_8_0: i64 = 64;
        // D s_8_1: read-var t:i
        let s_8_1: i128 = fn_state.t;
        // D s_8_2: call X_read(s_8_1, s_8_0)
        let s_8_2: Bits = X_read(state, tracer, s_8_1, s_8_0);
        // D s_8_3: cast reint s_8_2 -> u64
        let s_8_3: u64 = (s_8_2.value() as u64);
        // C s_8_4: const #28s : i
        let s_8_4: i128 = 28;
        // D s_8_5: cast zx s_8_3 -> bv
        let s_8_5: Bits = Bits::new(s_8_3 as u128, 64u16);
        // C s_8_6: const #1s : i64
        let s_8_6: i64 = 1;
        // C s_8_7: cast zx s_8_6 -> i
        let s_8_7: i128 = (i128::try_from(s_8_6).unwrap());
        // C s_8_8: const #3s : i
        let s_8_8: i128 = 3;
        // C s_8_9: add s_8_8 s_8_7
        let s_8_9: i128 = (s_8_8 + s_8_7);
        // D s_8_10: bit-extract s_8_5 s_8_4 s_8_9
        let s_8_10: Bits = (Bits::new(
            ((s_8_5) >> (s_8_4)).value(),
            u16::try_from(s_8_9).unwrap(),
        ));
        // D s_8_11: cast reint s_8_10 -> u8
        let s_8_11: u8 = (s_8_10.value() as u8);
        // C s_8_12: const #3s : i
        let s_8_12: i128 = 3;
        // D s_8_13: cast zx s_8_11 -> bv
        let s_8_13: Bits = Bits::new(s_8_11 as u128, 4u16);
        // C s_8_14: const #1s : i64
        let s_8_14: i64 = 1;
        // C s_8_15: cast zx s_8_14 -> i
        let s_8_15: i128 = (i128::try_from(s_8_14).unwrap());
        // C s_8_16: const #0s : i
        let s_8_16: i128 = 0;
        // C s_8_17: add s_8_16 s_8_15
        let s_8_17: i128 = (s_8_16 + s_8_15);
        // D s_8_18: bit-extract s_8_13 s_8_12 s_8_17
        let s_8_18: Bits = (Bits::new(
            ((s_8_13) >> (s_8_12)).value(),
            u16::try_from(s_8_17).unwrap(),
        ));
        // D s_8_19: cast reint s_8_18 -> u8
        let s_8_19: bool = ((s_8_18.value()) != 0);
        // C s_8_20: const #16984u : u32
        let s_8_20: u32 = 16984;
        // N s_8_21: write-reg s_8_20 <= s_8_19
        let s_8_21: () = {
            state.write_register::<bool>(s_8_20 as isize, s_8_19);
            tracer.write_register(s_8_20 as isize, s_8_19);
        };
        // C s_8_22: const #2s : i
        let s_8_22: i128 = 2;
        // D s_8_23: cast zx s_8_11 -> bv
        let s_8_23: Bits = Bits::new(s_8_11 as u128, 4u16);
        // C s_8_24: const #1s : i64
        let s_8_24: i64 = 1;
        // C s_8_25: cast zx s_8_24 -> i
        let s_8_25: i128 = (i128::try_from(s_8_24).unwrap());
        // C s_8_26: const #0s : i
        let s_8_26: i128 = 0;
        // C s_8_27: add s_8_26 s_8_25
        let s_8_27: i128 = (s_8_26 + s_8_25);
        // D s_8_28: bit-extract s_8_23 s_8_22 s_8_27
        let s_8_28: Bits = (Bits::new(
            ((s_8_23) >> (s_8_22)).value(),
            u16::try_from(s_8_27).unwrap(),
        ));
        // D s_8_29: cast reint s_8_28 -> u8
        let s_8_29: bool = ((s_8_28.value()) != 0);
        // C s_8_30: const #16997u : u32
        let s_8_30: u32 = 16997;
        // N s_8_31: write-reg s_8_30 <= s_8_29
        let s_8_31: () = {
            state.write_register::<bool>(s_8_30 as isize, s_8_29);
            tracer.write_register(s_8_30 as isize, s_8_29);
        };
        // C s_8_32: const #1s : i
        let s_8_32: i128 = 1;
        // D s_8_33: cast zx s_8_11 -> bv
        let s_8_33: Bits = Bits::new(s_8_11 as u128, 4u16);
        // C s_8_34: const #1s : i64
        let s_8_34: i64 = 1;
        // C s_8_35: cast zx s_8_34 -> i
        let s_8_35: i128 = (i128::try_from(s_8_34).unwrap());
        // C s_8_36: const #0s : i
        let s_8_36: i128 = 0;
        // C s_8_37: add s_8_36 s_8_35
        let s_8_37: i128 = (s_8_36 + s_8_35);
        // D s_8_38: bit-extract s_8_33 s_8_32 s_8_37
        let s_8_38: Bits = (Bits::new(
            ((s_8_33) >> (s_8_32)).value(),
            u16::try_from(s_8_37).unwrap(),
        ));
        // D s_8_39: cast reint s_8_38 -> u8
        let s_8_39: bool = ((s_8_38.value()) != 0);
        // C s_8_40: const #16971u : u32
        let s_8_40: u32 = 16971;
        // N s_8_41: write-reg s_8_40 <= s_8_39
        let s_8_41: () = {
            state.write_register::<bool>(s_8_40 as isize, s_8_39);
            tracer.write_register(s_8_40 as isize, s_8_39);
        };
        // C s_8_42: const #0s : i
        let s_8_42: i128 = 0;
        // D s_8_43: cast zx s_8_11 -> bv
        let s_8_43: Bits = Bits::new(s_8_11 as u128, 4u16);
        // C s_8_44: const #1s : i64
        let s_8_44: i64 = 1;
        // C s_8_45: cast zx s_8_44 -> i
        let s_8_45: i128 = (i128::try_from(s_8_44).unwrap());
        // C s_8_46: const #0s : i
        let s_8_46: i128 = 0;
        // C s_8_47: add s_8_46 s_8_45
        let s_8_47: i128 = (s_8_46 + s_8_45);
        // D s_8_48: bit-extract s_8_43 s_8_42 s_8_47
        let s_8_48: Bits = (Bits::new(
            ((s_8_43) >> (s_8_42)).value(),
            u16::try_from(s_8_47).unwrap(),
        ));
        // D s_8_49: cast reint s_8_48 -> u8
        let s_8_49: bool = ((s_8_48.value()) != 0);
        // C s_8_50: const #16996u : u32
        let s_8_50: u32 = 16996;
        // N s_8_51: write-reg s_8_50 <= s_8_49
        let s_8_51: () = {
            state.write_register::<bool>(s_8_50 as isize, s_8_49);
            tracer.write_register(s_8_50 as isize, s_8_49);
        };
        // N s_8_52: return
        return;
    }
}
