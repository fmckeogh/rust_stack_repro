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
use Rmode_set::*;
use R_set::*;
use common::*;
pub fn AArch32_ResetGeneralRegisters<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_31275: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        u_1462: i64,
        u_1461: i64,
        i: i64,
        gs_31275: (),
    }
    let fn_state = FunctionState {
        gs_31275,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0s : i64
        let s_0_0: i64 = 0;
        // D s_0_1: write-var i <= s_0_0
        fn_state.i = s_0_0;
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var i:i64
        let s_1_0: i64 = fn_state.i;
        // C s_1_1: const #7s : i64
        let s_1_1: i64 = 7;
        // D s_1_2: cmp-gt s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) > (s_1_1));
        // N s_1_3: branch s_1_2 b3 b2
        if s_1_2 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #32s : i64
        let s_2_0: i64 = 32;
        // C s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // S s_2_2: call __UNKNOWN_bits(s_2_1)
        let s_2_2: Bits = u__UNKNOWN_bits(state, tracer, s_2_1);
        // S s_2_3: cast reint s_2_2 -> u32
        let s_2_3: u32 = (s_2_2.value() as u32);
        // D s_2_4: read-var i:i64
        let s_2_4: i64 = fn_state.i;
        // D s_2_5: cast zx s_2_4 -> i
        let s_2_5: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_6: call R_set(s_2_5, s_2_3)
        let s_2_6: () = R_set(state, tracer, s_2_5, s_2_3);
        // D s_2_7: read-var i:i64
        let s_2_7: i64 = fn_state.i;
        // C s_2_8: const #1s : i64
        let s_2_8: i64 = 1;
        // D s_2_9: add s_2_7 s_2_8
        let s_2_9: i64 = (s_2_7 + s_2_8);
        // D s_2_10: write-var i <= s_2_9
        fn_state.i = s_2_9;
        // N s_2_11: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #8s : i64
        let s_3_0: i64 = 8;
        // D s_3_1: write-var u#1461 <= s_3_0
        fn_state.u_1461 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var u#1461:i64
        let s_4_0: i64 = fn_state.u_1461;
        // C s_4_1: const #12s : i64
        let s_4_1: i64 = 12;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b6 b5
        if s_4_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #32s : i64
        let s_5_0: i64 = 32;
        // C s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // S s_5_2: call __UNKNOWN_bits(s_5_1)
        let s_5_2: Bits = u__UNKNOWN_bits(state, tracer, s_5_1);
        // S s_5_3: cast reint s_5_2 -> u32
        let s_5_3: u32 = (s_5_2.value() as u32);
        // D s_5_4: read-var u#1461:i64
        let s_5_4: i64 = fn_state.u_1461;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // C s_5_6: const #352u : u32
        let s_5_6: u32 = 352;
        // D s_5_7: read-reg s_5_6:u8
        let s_5_7: u8 = {
            let value = state.read_register::<u8>(s_5_6 as isize);
            tracer.read_register(s_5_6 as isize, value);
            value
        };
        // D s_5_8: call Rmode_set(s_5_5, s_5_7, s_5_3)
        let s_5_8: () = Rmode_set(state, tracer, s_5_5, s_5_7, s_5_3);
        // C s_5_9: const #32s : i64
        let s_5_9: i64 = 32;
        // C s_5_10: cast zx s_5_9 -> i
        let s_5_10: i128 = (i128::try_from(s_5_9).unwrap());
        // S s_5_11: call __UNKNOWN_bits(s_5_10)
        let s_5_11: Bits = u__UNKNOWN_bits(state, tracer, s_5_10);
        // S s_5_12: cast reint s_5_11 -> u32
        let s_5_12: u32 = (s_5_11.value() as u32);
        // D s_5_13: read-var u#1461:i64
        let s_5_13: i64 = fn_state.u_1461;
        // D s_5_14: cast zx s_5_13 -> i
        let s_5_14: i128 = (i128::try_from(s_5_13).unwrap());
        // C s_5_15: const #360u : u32
        let s_5_15: u32 = 360;
        // D s_5_16: read-reg s_5_15:u8
        let s_5_16: u8 = {
            let value = state.read_register::<u8>(s_5_15 as isize);
            tracer.read_register(s_5_15 as isize, value);
            value
        };
        // D s_5_17: call Rmode_set(s_5_14, s_5_16, s_5_12)
        let s_5_17: () = Rmode_set(state, tracer, s_5_14, s_5_16, s_5_12);
        // D s_5_18: read-var u#1461:i64
        let s_5_18: i64 = fn_state.u_1461;
        // C s_5_19: const #1s : i64
        let s_5_19: i64 = 1;
        // D s_5_20: add s_5_18 s_5_19
        let s_5_20: i64 = (s_5_18 + s_5_19);
        // D s_5_21: write-var u#1461 <= s_5_20
        fn_state.u_1461 = s_5_20;
        // N s_5_22: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #432u : u32
        let s_6_0: u32 = 432;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // C s_6_2: const #2u : u8
        let s_6_2: u8 = 2;
        // D s_6_3: cmp-lt s_6_1 s_6_2
        let s_6_3: bool = ((s_6_1) < (s_6_2));
        // N s_6_4: branch s_6_3 b15 b7
        if s_6_3 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #13s : i64
        let s_8_0: i64 = 13;
        // D s_8_1: write-var u#1462 <= s_8_0
        fn_state.u_1462 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var u#1462:i64
        let s_9_0: i64 = fn_state.u_1462;
        // C s_9_1: const #14s : i64
        let s_9_1: i64 = 14;
        // D s_9_2: cmp-gt s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) > (s_9_1));
        // N s_9_3: branch s_9_2 b14 b10
        if s_9_2 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #32s : i64
        let s_10_0: i64 = 32;
        // C s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // S s_10_2: call __UNKNOWN_bits(s_10_1)
        let s_10_2: Bits = u__UNKNOWN_bits(state, tracer, s_10_1);
        // S s_10_3: cast reint s_10_2 -> u32
        let s_10_3: u32 = (s_10_2.value() as u32);
        // D s_10_4: read-var u#1462:i64
        let s_10_4: i64 = fn_state.u_1462;
        // D s_10_5: cast zx s_10_4 -> i
        let s_10_5: i128 = (i128::try_from(s_10_4).unwrap());
        // C s_10_6: const #352u : u32
        let s_10_6: u32 = 352;
        // D s_10_7: read-reg s_10_6:u8
        let s_10_7: u8 = {
            let value = state.read_register::<u8>(s_10_6 as isize);
            tracer.read_register(s_10_6 as isize, value);
            value
        };
        // D s_10_8: call Rmode_set(s_10_5, s_10_7, s_10_3)
        let s_10_8: () = Rmode_set(state, tracer, s_10_5, s_10_7, s_10_3);
        // C s_10_9: const #32s : i64
        let s_10_9: i64 = 32;
        // C s_10_10: cast zx s_10_9 -> i
        let s_10_10: i128 = (i128::try_from(s_10_9).unwrap());
        // S s_10_11: call __UNKNOWN_bits(s_10_10)
        let s_10_11: Bits = u__UNKNOWN_bits(state, tracer, s_10_10);
        // S s_10_12: cast reint s_10_11 -> u32
        let s_10_12: u32 = (s_10_11.value() as u32);
        // D s_10_13: read-var u#1462:i64
        let s_10_13: i64 = fn_state.u_1462;
        // D s_10_14: cast zx s_10_13 -> i
        let s_10_14: i128 = (i128::try_from(s_10_13).unwrap());
        // C s_10_15: const #360u : u32
        let s_10_15: u32 = 360;
        // D s_10_16: read-reg s_10_15:u8
        let s_10_16: u8 = {
            let value = state.read_register::<u8>(s_10_15 as isize);
            tracer.read_register(s_10_15 as isize, value);
            value
        };
        // D s_10_17: call Rmode_set(s_10_14, s_10_16, s_10_12)
        let s_10_17: () = Rmode_set(state, tracer, s_10_14, s_10_16, s_10_12);
        // C s_10_18: const #32s : i64
        let s_10_18: i64 = 32;
        // C s_10_19: cast zx s_10_18 -> i
        let s_10_19: i128 = (i128::try_from(s_10_18).unwrap());
        // S s_10_20: call __UNKNOWN_bits(s_10_19)
        let s_10_20: Bits = u__UNKNOWN_bits(state, tracer, s_10_19);
        // S s_10_21: cast reint s_10_20 -> u32
        let s_10_21: u32 = (s_10_20.value() as u32);
        // D s_10_22: read-var u#1462:i64
        let s_10_22: i64 = fn_state.u_1462;
        // D s_10_23: cast zx s_10_22 -> i
        let s_10_23: i128 = (i128::try_from(s_10_22).unwrap());
        // C s_10_24: const #368u : u32
        let s_10_24: u32 = 368;
        // D s_10_25: read-reg s_10_24:u8
        let s_10_25: u8 = {
            let value = state.read_register::<u8>(s_10_24 as isize);
            tracer.read_register(s_10_24 as isize, value);
            value
        };
        // D s_10_26: call Rmode_set(s_10_23, s_10_25, s_10_21)
        let s_10_26: () = Rmode_set(state, tracer, s_10_23, s_10_25, s_10_21);
        // C s_10_27: const #32s : i64
        let s_10_27: i64 = 32;
        // C s_10_28: cast zx s_10_27 -> i
        let s_10_28: i128 = (i128::try_from(s_10_27).unwrap());
        // S s_10_29: call __UNKNOWN_bits(s_10_28)
        let s_10_29: Bits = u__UNKNOWN_bits(state, tracer, s_10_28);
        // S s_10_30: cast reint s_10_29 -> u32
        let s_10_30: u32 = (s_10_29.value() as u32);
        // D s_10_31: read-var u#1462:i64
        let s_10_31: i64 = fn_state.u_1462;
        // D s_10_32: cast zx s_10_31 -> i
        let s_10_32: i128 = (i128::try_from(s_10_31).unwrap());
        // C s_10_33: const #376u : u32
        let s_10_33: u32 = 376;
        // D s_10_34: read-reg s_10_33:u8
        let s_10_34: u8 = {
            let value = state.read_register::<u8>(s_10_33 as isize);
            tracer.read_register(s_10_33 as isize, value);
            value
        };
        // D s_10_35: call Rmode_set(s_10_32, s_10_34, s_10_30)
        let s_10_35: () = Rmode_set(state, tracer, s_10_32, s_10_34, s_10_30);
        // C s_10_36: const #32s : i64
        let s_10_36: i64 = 32;
        // C s_10_37: cast zx s_10_36 -> i
        let s_10_37: i128 = (i128::try_from(s_10_36).unwrap());
        // S s_10_38: call __UNKNOWN_bits(s_10_37)
        let s_10_38: Bits = u__UNKNOWN_bits(state, tracer, s_10_37);
        // S s_10_39: cast reint s_10_38 -> u32
        let s_10_39: u32 = (s_10_38.value() as u32);
        // D s_10_40: read-var u#1462:i64
        let s_10_40: i64 = fn_state.u_1462;
        // D s_10_41: cast zx s_10_40 -> i
        let s_10_41: i128 = (i128::try_from(s_10_40).unwrap());
        // C s_10_42: const #392u : u32
        let s_10_42: u32 = 392;
        // D s_10_43: read-reg s_10_42:u8
        let s_10_43: u8 = {
            let value = state.read_register::<u8>(s_10_42 as isize);
            tracer.read_register(s_10_42 as isize, value);
            value
        };
        // D s_10_44: call Rmode_set(s_10_41, s_10_43, s_10_39)
        let s_10_44: () = Rmode_set(state, tracer, s_10_41, s_10_43, s_10_39);
        // C s_10_45: const #32s : i64
        let s_10_45: i64 = 32;
        // C s_10_46: cast zx s_10_45 -> i
        let s_10_46: i128 = (i128::try_from(s_10_45).unwrap());
        // S s_10_47: call __UNKNOWN_bits(s_10_46)
        let s_10_47: Bits = u__UNKNOWN_bits(state, tracer, s_10_46);
        // S s_10_48: cast reint s_10_47 -> u32
        let s_10_48: u32 = (s_10_47.value() as u32);
        // D s_10_49: read-var u#1462:i64
        let s_10_49: i64 = fn_state.u_1462;
        // D s_10_50: cast zx s_10_49 -> i
        let s_10_50: i128 = (i128::try_from(s_10_49).unwrap());
        // C s_10_51: const #408u : u32
        let s_10_51: u32 = 408;
        // D s_10_52: read-reg s_10_51:u8
        let s_10_52: u8 = {
            let value = state.read_register::<u8>(s_10_51 as isize);
            tracer.read_register(s_10_51 as isize, value);
            value
        };
        // D s_10_53: call Rmode_set(s_10_50, s_10_52, s_10_48)
        let s_10_53: () = Rmode_set(state, tracer, s_10_50, s_10_52, s_10_48);
        // C s_10_54: const #424u : u32
        let s_10_54: u32 = 424;
        // D s_10_55: read-reg s_10_54:u8
        let s_10_55: u8 = {
            let value = state.read_register::<u8>(s_10_54 as isize);
            tracer.read_register(s_10_54 as isize, value);
            value
        };
        // C s_10_56: const #2u : u8
        let s_10_56: u8 = 2;
        // D s_10_57: cmp-lt s_10_55 s_10_56
        let s_10_57: bool = ((s_10_55) < (s_10_56));
        // N s_10_58: branch s_10_57 b13 b11
        if s_10_57 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var u#1462:i64
        let s_12_0: i64 = fn_state.u_1462;
        // C s_12_1: const #1s : i64
        let s_12_1: i64 = 1;
        // D s_12_2: add s_12_0 s_12_1
        let s_12_2: i64 = (s_12_0 + s_12_1);
        // D s_12_3: write-var u#1462 <= s_12_2
        fn_state.u_1462 = s_12_2;
        // N s_12_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #32s : i64
        let s_13_0: i64 = 32;
        // C s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // S s_13_2: call __UNKNOWN_bits(s_13_1)
        let s_13_2: Bits = u__UNKNOWN_bits(state, tracer, s_13_1);
        // S s_13_3: cast reint s_13_2 -> u32
        let s_13_3: u32 = (s_13_2.value() as u32);
        // D s_13_4: read-var u#1462:i64
        let s_13_4: i64 = fn_state.u_1462;
        // D s_13_5: cast zx s_13_4 -> i
        let s_13_5: i128 = (i128::try_from(s_13_4).unwrap());
        // C s_13_6: const #384u : u32
        let s_13_6: u32 = 384;
        // D s_13_7: read-reg s_13_6:u8
        let s_13_7: u8 = {
            let value = state.read_register::<u8>(s_13_6 as isize);
            tracer.read_register(s_13_6 as isize, value);
            value
        };
        // D s_13_8: call Rmode_set(s_13_5, s_13_7, s_13_3)
        let s_13_8: () = Rmode_set(state, tracer, s_13_5, s_13_7, s_13_3);
        // N s_13_9: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #32s : i64
        let s_15_0: i64 = 32;
        // C s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // S s_15_2: call __UNKNOWN_bits(s_15_1)
        let s_15_2: Bits = u__UNKNOWN_bits(state, tracer, s_15_1);
        // S s_15_3: cast reint s_15_2 -> u32
        let s_15_3: u32 = (s_15_2.value() as u32);
        // C s_15_4: const #13s : i
        let s_15_4: i128 = 13;
        // C s_15_5: const #400u : u32
        let s_15_5: u32 = 400;
        // D s_15_6: read-reg s_15_5:u8
        let s_15_6: u8 = {
            let value = state.read_register::<u8>(s_15_5 as isize);
            tracer.read_register(s_15_5 as isize, value);
            value
        };
        // D s_15_7: call Rmode_set(s_15_4, s_15_6, s_15_3)
        let s_15_7: () = Rmode_set(state, tracer, s_15_4, s_15_6, s_15_3);
        // N s_15_8: jump b8
        return block_8(state, tracer, fn_state);
    }
}
