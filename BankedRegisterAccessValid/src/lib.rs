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
use CurrentSecurityState::*;
use common::*;
pub fn BankedRegisterAccessValid<T: Tracer>(
    state: &mut State,
    tracer: &T,
    SYSm: u8,
    mode: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_31494: bool,
        gs_31490: bool,
        gs_31459: bool,
        gs_31495: bool,
        gs_31491: bool,
        gs_31498: bool,
        SYSm: u8,
        mode: u8,
    }
    let fn_state = FunctionState {
        SYSm,
        mode,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var SYSm:u8
        let s_0_0: u8 = fn_state.SYSm;
        // C s_0_1: const #2s : i
        let s_0_1: i128 = 2;
        // D s_0_2: cast zx s_0_0 -> bv
        let s_0_2: Bits = Bits::new(s_0_0 as u128, 5u16);
        // C s_0_3: const #1s : i64
        let s_0_3: i64 = 1;
        // C s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // C s_0_5: const #2s : i
        let s_0_5: i128 = 2;
        // C s_0_6: add s_0_5 s_0_4
        let s_0_6: i128 = (s_0_5 + s_0_4);
        // D s_0_7: bit-extract s_0_2 s_0_1 s_0_6
        let s_0_7: Bits = (Bits::new(
            ((s_0_2) >> (s_0_1)).value(),
            u16::try_from(s_0_6).unwrap(),
        ));
        // D s_0_8: cast reint s_0_7 -> u8
        let s_0_8: u8 = (s_0_7.value() as u8);
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 3u16);
        // C s_0_10: const #0u : u8
        let s_0_10: u8 = 0;
        // C s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 3u16);
        // D s_0_12: cmp-eq s_0_9 s_0_11
        let s_0_12: bool = ((s_0_9) == (s_0_11));
        // D s_0_13: not s_0_12
        let s_0_13: bool = !s_0_12;
        // N s_0_14: branch s_0_13 b4 b1
        if s_0_13 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var mode:u8
        let s_1_0: u8 = fn_state.mode;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 5u16);
        // C s_1_2: const #360u : u32
        let s_1_2: u32 = 360;
        // D s_1_3: read-reg s_1_2:u8
        let s_1_3: u8 = {
            let value = state.read_register::<u8>(s_1_2 as isize);
            tracer.read_register(s_1_2 as isize, value);
            value
        };
        // D s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 5u16);
        // D s_1_5: cmp-ne s_1_1 s_1_4
        let s_1_5: bool = ((s_1_1) != (s_1_4));
        // N s_1_6: branch s_1_5 b3 b2
        if s_1_5 {
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
        // N s_2_0: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: panic
        panic!("{:?}", ());
        // N s_3_1: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var SYSm:u8
        let s_4_0: u8 = fn_state.SYSm;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 5u16);
        // C s_4_2: const #4u : u8
        let s_4_2: u8 = 4;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 5u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // D s_4_5: not s_4_4
        let s_4_5: bool = !s_4_4;
        // N s_4_6: branch s_4_5 b8 b5
        if s_4_5 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var mode:u8
        let s_5_0: u8 = fn_state.mode;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 5u16);
        // C s_5_2: const #360u : u32
        let s_5_2: u32 = 360;
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
        // N s_5_6: branch s_5_5 b7 b6
        if s_5_5 {
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
        // N s_6_0: return
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
        // D s_8_0: read-var SYSm:u8
        let s_8_0: u8 = fn_state.SYSm;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 5u16);
        // C s_8_2: const #5u : u8
        let s_8_2: u8 = 5;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 5u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // D s_8_5: not s_8_4
        let s_8_5: bool = !s_8_4;
        // N s_8_6: branch s_8_5 b12 b9
        if s_8_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var mode:u8
        let s_9_0: u8 = fn_state.mode;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 5u16);
        // C s_9_2: const #416u : u32
        let s_9_2: u32 = 416;
        // D s_9_3: read-reg s_9_2:u8
        let s_9_3: u8 = {
            let value = state.read_register::<u8>(s_9_2 as isize);
            tracer.read_register(s_9_2 as isize, value);
            value
        };
        // D s_9_4: cast zx s_9_3 -> bv
        let s_9_4: Bits = Bits::new(s_9_3 as u128, 5u16);
        // D s_9_5: cmp-eq s_9_1 s_9_4
        let s_9_5: bool = ((s_9_1) == (s_9_4));
        // N s_9_6: branch s_9_5 b11 b10
        if s_9_5 {
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
        // N s_10_0: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: panic
        panic!("{:?}", ());
        // N s_11_1: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var SYSm:u8
        let s_12_0: u8 = fn_state.SYSm;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 5u16);
        // C s_12_2: const #6u : u8
        let s_12_2: u8 = 6;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 5u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: not s_12_4
        let s_12_5: bool = !s_12_4;
        // N s_12_6: branch s_12_5 b19 b13
        if s_12_5 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var mode:u8
        let s_13_0: u8 = fn_state.mode;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 5u16);
        // C s_13_2: const #400u : u32
        let s_13_2: u32 = 400;
        // D s_13_3: read-reg s_13_2:u8
        let s_13_3: u8 = {
            let value = state.read_register::<u8>(s_13_2 as isize);
            tracer.read_register(s_13_2 as isize, value);
            value
        };
        // D s_13_4: cast zx s_13_3 -> bv
        let s_13_4: Bits = Bits::new(s_13_3 as u128, 5u16);
        // D s_13_5: cmp-eq s_13_1 s_13_4
        let s_13_5: bool = ((s_13_1) == (s_13_4));
        // N s_13_6: branch s_13_5 b18 b14
        if s_13_5 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var mode:u8
        let s_14_0: u8 = fn_state.mode;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 5u16);
        // C s_14_2: const #416u : u32
        let s_14_2: u32 = 416;
        // D s_14_3: read-reg s_14_2:u8
        let s_14_3: u8 = {
            let value = state.read_register::<u8>(s_14_2 as isize);
            tracer.read_register(s_14_2 as isize, value);
            value
        };
        // D s_14_4: cast zx s_14_3 -> bv
        let s_14_4: Bits = Bits::new(s_14_3 as u128, 5u16);
        // D s_14_5: cmp-eq s_14_1 s_14_4
        let s_14_5: bool = ((s_14_1) == (s_14_4));
        // D s_14_6: write-var gs#31459 <= s_14_5
        fn_state.gs_31459 = s_14_5;
        // N s_14_7: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#31459:u8
        let s_15_0: bool = fn_state.gs_31459;
        // N s_15_1: branch s_15_0 b17 b16
        if s_15_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: panic
        panic!("{:?}", ());
        // N s_17_1: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#31459 <= s_18_0
        fn_state.gs_31459 = s_18_0;
        // N s_18_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var SYSm:u8
        let s_19_0: u8 = fn_state.SYSm;
        // C s_19_1: const #2s : i
        let s_19_1: i128 = 2;
        // D s_19_2: cast zx s_19_0 -> bv
        let s_19_2: Bits = Bits::new(s_19_0 as u128, 5u16);
        // C s_19_3: const #1s : i64
        let s_19_3: i64 = 1;
        // C s_19_4: cast zx s_19_3 -> i
        let s_19_4: i128 = (i128::try_from(s_19_3).unwrap());
        // C s_19_5: const #2s : i
        let s_19_5: i128 = 2;
        // C s_19_6: add s_19_5 s_19_4
        let s_19_6: i128 = (s_19_5 + s_19_4);
        // D s_19_7: bit-extract s_19_2 s_19_1 s_19_6
        let s_19_7: Bits = (Bits::new(
            ((s_19_2) >> (s_19_1)).value(),
            u16::try_from(s_19_6).unwrap(),
        ));
        // D s_19_8: cast reint s_19_7 -> u8
        let s_19_8: u8 = (s_19_7.value() as u8);
        // D s_19_9: cast zx s_19_8 -> bv
        let s_19_9: Bits = Bits::new(s_19_8 as u128, 3u16);
        // C s_19_10: const #2u : u8
        let s_19_10: u8 = 2;
        // C s_19_11: cast zx s_19_10 -> bv
        let s_19_11: Bits = Bits::new(s_19_10 as u128, 3u16);
        // D s_19_12: cmp-eq s_19_9 s_19_11
        let s_19_12: bool = ((s_19_9) == (s_19_11));
        // D s_19_13: not s_19_12
        let s_19_13: bool = !s_19_12;
        // N s_19_14: branch s_19_13 b23 b20
        if s_19_13 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var mode:u8
        let s_20_0: u8 = fn_state.mode;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 5u16);
        // C s_20_2: const #360u : u32
        let s_20_2: u32 = 360;
        // D s_20_3: read-reg s_20_2:u8
        let s_20_3: u8 = {
            let value = state.read_register::<u8>(s_20_2 as isize);
            tracer.read_register(s_20_2 as isize, value);
            value
        };
        // D s_20_4: cast zx s_20_3 -> bv
        let s_20_4: Bits = Bits::new(s_20_3 as u128, 5u16);
        // D s_20_5: cmp-eq s_20_1 s_20_4
        let s_20_5: bool = ((s_20_1) == (s_20_4));
        // N s_20_6: branch s_20_5 b22 b21
        if s_20_5 {
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
        // N s_21_0: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: panic
        panic!("{:?}", ());
        // N s_22_1: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var SYSm:u8
        let s_23_0: u8 = fn_state.SYSm;
        // C s_23_1: const #1s : i
        let s_23_1: i128 = 1;
        // D s_23_2: cast zx s_23_0 -> bv
        let s_23_2: Bits = Bits::new(s_23_0 as u128, 5u16);
        // C s_23_3: const #1s : i64
        let s_23_3: i64 = 1;
        // C s_23_4: cast zx s_23_3 -> i
        let s_23_4: i128 = (i128::try_from(s_23_3).unwrap());
        // C s_23_5: const #3s : i
        let s_23_5: i128 = 3;
        // C s_23_6: add s_23_5 s_23_4
        let s_23_6: i128 = (s_23_5 + s_23_4);
        // D s_23_7: bit-extract s_23_2 s_23_1 s_23_6
        let s_23_7: Bits = (Bits::new(
            ((s_23_2) >> (s_23_1)).value(),
            u16::try_from(s_23_6).unwrap(),
        ));
        // D s_23_8: cast reint s_23_7 -> u8
        let s_23_8: u8 = (s_23_7.value() as u8);
        // D s_23_9: cast zx s_23_8 -> bv
        let s_23_9: Bits = Bits::new(s_23_8 as u128, 4u16);
        // C s_23_10: const #6u : u8
        let s_23_10: u8 = 6;
        // C s_23_11: cast zx s_23_10 -> bv
        let s_23_11: Bits = Bits::new(s_23_10 as u128, 4u16);
        // D s_23_12: cmp-eq s_23_9 s_23_11
        let s_23_12: bool = ((s_23_9) == (s_23_11));
        // D s_23_13: not s_23_12
        let s_23_13: bool = !s_23_12;
        // N s_23_14: branch s_23_13 b27 b24
        if s_23_13 {
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
        // D s_24_0: read-var mode:u8
        let s_24_0: u8 = fn_state.mode;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 5u16);
        // C s_24_2: const #360u : u32
        let s_24_2: u32 = 360;
        // D s_24_3: read-reg s_24_2:u8
        let s_24_3: u8 = {
            let value = state.read_register::<u8>(s_24_2 as isize);
            tracer.read_register(s_24_2 as isize, value);
            value
        };
        // D s_24_4: cast zx s_24_3 -> bv
        let s_24_4: Bits = Bits::new(s_24_3 as u128, 5u16);
        // D s_24_5: cmp-eq s_24_1 s_24_4
        let s_24_5: bool = ((s_24_1) == (s_24_4));
        // N s_24_6: branch s_24_5 b26 b25
        if s_24_5 {
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
        // N s_25_0: return
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
        // D s_27_0: read-var SYSm:u8
        let s_27_0: u8 = fn_state.SYSm;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 5u16);
        // C s_27_2: const #14u : u8
        let s_27_2: u8 = 14;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 5u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: not s_27_4
        let s_27_5: bool = !s_27_4;
        // N s_27_6: branch s_27_5 b31 b28
        if s_27_5 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var mode:u8
        let s_28_0: u8 = fn_state.mode;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 5u16);
        // C s_28_2: const #360u : u32
        let s_28_2: u32 = 360;
        // D s_28_3: read-reg s_28_2:u8
        let s_28_3: u8 = {
            let value = state.read_register::<u8>(s_28_2 as isize);
            tracer.read_register(s_28_2 as isize, value);
            value
        };
        // D s_28_4: cast zx s_28_3 -> bv
        let s_28_4: Bits = Bits::new(s_28_3 as u128, 5u16);
        // D s_28_5: cmp-eq s_28_1 s_28_4
        let s_28_5: bool = ((s_28_1) == (s_28_4));
        // N s_28_6: branch s_28_5 b30 b29
        if s_28_5 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: panic
        panic!("{:?}", ());
        // N s_30_1: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var SYSm:u8
        let s_31_0: u8 = fn_state.SYSm;
        // C s_31_1: const #1s : i
        let s_31_1: i128 = 1;
        // D s_31_2: cast zx s_31_0 -> bv
        let s_31_2: Bits = Bits::new(s_31_0 as u128, 5u16);
        // C s_31_3: const #1s : i64
        let s_31_3: i64 = 1;
        // C s_31_4: cast zx s_31_3 -> i
        let s_31_4: i128 = (i128::try_from(s_31_3).unwrap());
        // C s_31_5: const #3s : i
        let s_31_5: i128 = 3;
        // C s_31_6: add s_31_5 s_31_4
        let s_31_6: i128 = (s_31_5 + s_31_4);
        // D s_31_7: bit-extract s_31_2 s_31_1 s_31_6
        let s_31_7: Bits = (Bits::new(
            ((s_31_2) >> (s_31_1)).value(),
            u16::try_from(s_31_6).unwrap(),
        ));
        // D s_31_8: cast reint s_31_7 -> u8
        let s_31_8: u8 = (s_31_7.value() as u8);
        // D s_31_9: cast zx s_31_8 -> bv
        let s_31_9: Bits = Bits::new(s_31_8 as u128, 4u16);
        // C s_31_10: const #8u : u8
        let s_31_10: u8 = 8;
        // C s_31_11: cast zx s_31_10 -> bv
        let s_31_11: Bits = Bits::new(s_31_10 as u128, 4u16);
        // D s_31_12: cmp-eq s_31_9 s_31_11
        let s_31_12: bool = ((s_31_9) == (s_31_11));
        // D s_31_13: not s_31_12
        let s_31_13: bool = !s_31_12;
        // N s_31_14: branch s_31_13 b35 b32
        if s_31_13 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var mode:u8
        let s_32_0: u8 = fn_state.mode;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 5u16);
        // C s_32_2: const #368u : u32
        let s_32_2: u32 = 368;
        // D s_32_3: read-reg s_32_2:u8
        let s_32_3: u8 = {
            let value = state.read_register::<u8>(s_32_2 as isize);
            tracer.read_register(s_32_2 as isize, value);
            value
        };
        // D s_32_4: cast zx s_32_3 -> bv
        let s_32_4: Bits = Bits::new(s_32_3 as u128, 5u16);
        // D s_32_5: cmp-eq s_32_1 s_32_4
        let s_32_5: bool = ((s_32_1) == (s_32_4));
        // N s_32_6: branch s_32_5 b34 b33
        if s_32_5 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_33_0: return
        return;
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
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var SYSm:u8
        let s_35_0: u8 = fn_state.SYSm;
        // C s_35_1: const #1s : i
        let s_35_1: i128 = 1;
        // D s_35_2: cast zx s_35_0 -> bv
        let s_35_2: Bits = Bits::new(s_35_0 as u128, 5u16);
        // C s_35_3: const #1s : i64
        let s_35_3: i64 = 1;
        // C s_35_4: cast zx s_35_3 -> i
        let s_35_4: i128 = (i128::try_from(s_35_3).unwrap());
        // C s_35_5: const #3s : i
        let s_35_5: i128 = 3;
        // C s_35_6: add s_35_5 s_35_4
        let s_35_6: i128 = (s_35_5 + s_35_4);
        // D s_35_7: bit-extract s_35_2 s_35_1 s_35_6
        let s_35_7: Bits = (Bits::new(
            ((s_35_2) >> (s_35_1)).value(),
            u16::try_from(s_35_6).unwrap(),
        ));
        // D s_35_8: cast reint s_35_7 -> u8
        let s_35_8: u8 = (s_35_7.value() as u8);
        // D s_35_9: cast zx s_35_8 -> bv
        let s_35_9: Bits = Bits::new(s_35_8 as u128, 4u16);
        // C s_35_10: const #9u : u8
        let s_35_10: u8 = 9;
        // C s_35_11: cast zx s_35_10 -> bv
        let s_35_11: Bits = Bits::new(s_35_10 as u128, 4u16);
        // D s_35_12: cmp-eq s_35_9 s_35_11
        let s_35_12: bool = ((s_35_9) == (s_35_11));
        // D s_35_13: not s_35_12
        let s_35_13: bool = !s_35_12;
        // N s_35_14: branch s_35_13 b39 b36
        if s_35_13 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var mode:u8
        let s_36_0: u8 = fn_state.mode;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 5u16);
        // C s_36_2: const #376u : u32
        let s_36_2: u32 = 376;
        // D s_36_3: read-reg s_36_2:u8
        let s_36_3: u8 = {
            let value = state.read_register::<u8>(s_36_2 as isize);
            tracer.read_register(s_36_2 as isize, value);
            value
        };
        // D s_36_4: cast zx s_36_3 -> bv
        let s_36_4: Bits = Bits::new(s_36_3 as u128, 5u16);
        // D s_36_5: cmp-eq s_36_1 s_36_4
        let s_36_5: bool = ((s_36_1) == (s_36_4));
        // N s_36_6: branch s_36_5 b38 b37
        if s_36_5 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_37_0: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_38_0: panic
        panic!("{:?}", ());
        // N s_38_1: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var SYSm:u8
        let s_39_0: u8 = fn_state.SYSm;
        // C s_39_1: const #1s : i
        let s_39_1: i128 = 1;
        // D s_39_2: cast zx s_39_0 -> bv
        let s_39_2: Bits = Bits::new(s_39_0 as u128, 5u16);
        // C s_39_3: const #1s : i64
        let s_39_3: i64 = 1;
        // C s_39_4: cast zx s_39_3 -> i
        let s_39_4: i128 = (i128::try_from(s_39_3).unwrap());
        // C s_39_5: const #3s : i
        let s_39_5: i128 = 3;
        // C s_39_6: add s_39_5 s_39_4
        let s_39_6: i128 = (s_39_5 + s_39_4);
        // D s_39_7: bit-extract s_39_2 s_39_1 s_39_6
        let s_39_7: Bits = (Bits::new(
            ((s_39_2) >> (s_39_1)).value(),
            u16::try_from(s_39_6).unwrap(),
        ));
        // D s_39_8: cast reint s_39_7 -> u8
        let s_39_8: u8 = (s_39_7.value() as u8);
        // D s_39_9: cast zx s_39_8 -> bv
        let s_39_9: Bits = Bits::new(s_39_8 as u128, 4u16);
        // C s_39_10: const #10u : u8
        let s_39_10: u8 = 10;
        // C s_39_11: cast zx s_39_10 -> bv
        let s_39_11: Bits = Bits::new(s_39_10 as u128, 4u16);
        // D s_39_12: cmp-eq s_39_9 s_39_11
        let s_39_12: bool = ((s_39_9) == (s_39_11));
        // D s_39_13: not s_39_12
        let s_39_13: bool = !s_39_12;
        // N s_39_14: branch s_39_13 b43 b40
        if s_39_13 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var mode:u8
        let s_40_0: u8 = fn_state.mode;
        // D s_40_1: cast zx s_40_0 -> bv
        let s_40_1: Bits = Bits::new(s_40_0 as u128, 5u16);
        // C s_40_2: const #392u : u32
        let s_40_2: u32 = 392;
        // D s_40_3: read-reg s_40_2:u8
        let s_40_3: u8 = {
            let value = state.read_register::<u8>(s_40_2 as isize);
            tracer.read_register(s_40_2 as isize, value);
            value
        };
        // D s_40_4: cast zx s_40_3 -> bv
        let s_40_4: Bits = Bits::new(s_40_3 as u128, 5u16);
        // D s_40_5: cmp-eq s_40_1 s_40_4
        let s_40_5: bool = ((s_40_1) == (s_40_4));
        // N s_40_6: branch s_40_5 b42 b41
        if s_40_5 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_41_0: return
        return;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_42_0: panic
        panic!("{:?}", ());
        // N s_42_1: return
        return;
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var SYSm:u8
        let s_43_0: u8 = fn_state.SYSm;
        // C s_43_1: const #1s : i
        let s_43_1: i128 = 1;
        // D s_43_2: cast zx s_43_0 -> bv
        let s_43_2: Bits = Bits::new(s_43_0 as u128, 5u16);
        // C s_43_3: const #1s : i64
        let s_43_3: i64 = 1;
        // C s_43_4: cast zx s_43_3 -> i
        let s_43_4: i128 = (i128::try_from(s_43_3).unwrap());
        // C s_43_5: const #3s : i
        let s_43_5: i128 = 3;
        // C s_43_6: add s_43_5 s_43_4
        let s_43_6: i128 = (s_43_5 + s_43_4);
        // D s_43_7: bit-extract s_43_2 s_43_1 s_43_6
        let s_43_7: Bits = (Bits::new(
            ((s_43_2) >> (s_43_1)).value(),
            u16::try_from(s_43_6).unwrap(),
        ));
        // D s_43_8: cast reint s_43_7 -> u8
        let s_43_8: u8 = (s_43_7.value() as u8);
        // D s_43_9: cast zx s_43_8 -> bv
        let s_43_9: Bits = Bits::new(s_43_8 as u128, 4u16);
        // C s_43_10: const #11u : u8
        let s_43_10: u8 = 11;
        // C s_43_11: cast zx s_43_10 -> bv
        let s_43_11: Bits = Bits::new(s_43_10 as u128, 4u16);
        // D s_43_12: cmp-eq s_43_9 s_43_11
        let s_43_12: bool = ((s_43_9) == (s_43_11));
        // D s_43_13: not s_43_12
        let s_43_13: bool = !s_43_12;
        // N s_43_14: branch s_43_13 b47 b44
        if s_43_13 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var mode:u8
        let s_44_0: u8 = fn_state.mode;
        // D s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 5u16);
        // C s_44_2: const #408u : u32
        let s_44_2: u32 = 408;
        // D s_44_3: read-reg s_44_2:u8
        let s_44_3: u8 = {
            let value = state.read_register::<u8>(s_44_2 as isize);
            tracer.read_register(s_44_2 as isize, value);
            value
        };
        // D s_44_4: cast zx s_44_3 -> bv
        let s_44_4: Bits = Bits::new(s_44_3 as u128, 5u16);
        // D s_44_5: cmp-eq s_44_1 s_44_4
        let s_44_5: bool = ((s_44_1) == (s_44_4));
        // N s_44_6: branch s_44_5 b46 b45
        if s_44_5 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_45_0: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_46_0: panic
        panic!("{:?}", ());
        // N s_46_1: return
        return;
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var SYSm:u8
        let s_47_0: u8 = fn_state.SYSm;
        // C s_47_1: const #1s : i
        let s_47_1: i128 = 1;
        // D s_47_2: cast zx s_47_0 -> bv
        let s_47_2: Bits = Bits::new(s_47_0 as u128, 5u16);
        // C s_47_3: const #1s : i64
        let s_47_3: i64 = 1;
        // C s_47_4: cast zx s_47_3 -> i
        let s_47_4: i128 = (i128::try_from(s_47_3).unwrap());
        // C s_47_5: const #3s : i
        let s_47_5: i128 = 3;
        // C s_47_6: add s_47_5 s_47_4
        let s_47_6: i128 = (s_47_5 + s_47_4);
        // D s_47_7: bit-extract s_47_2 s_47_1 s_47_6
        let s_47_7: Bits = (Bits::new(
            ((s_47_2) >> (s_47_1)).value(),
            u16::try_from(s_47_6).unwrap(),
        ));
        // D s_47_8: cast reint s_47_7 -> u8
        let s_47_8: u8 = (s_47_7.value() as u8);
        // D s_47_9: cast zx s_47_8 -> bv
        let s_47_9: Bits = Bits::new(s_47_8 as u128, 4u16);
        // C s_47_10: const #14u : u8
        let s_47_10: u8 = 14;
        // C s_47_11: cast zx s_47_10 -> bv
        let s_47_11: Bits = Bits::new(s_47_10 as u128, 4u16);
        // D s_47_12: cmp-eq s_47_9 s_47_11
        let s_47_12: bool = ((s_47_9) == (s_47_11));
        // D s_47_13: not s_47_12
        let s_47_13: bool = !s_47_12;
        // N s_47_14: branch s_47_13 b57 b48
        if s_47_13 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #424u : u32
        let s_48_0: u32 = 424;
        // D s_48_1: read-reg s_48_0:u8
        let s_48_1: u8 = {
            let value = state.read_register::<u8>(s_48_0 as isize);
            tracer.read_register(s_48_0 as isize, value);
            value
        };
        // C s_48_2: const #2u : u8
        let s_48_2: u8 = 2;
        // D s_48_3: cmp-lt s_48_1 s_48_2
        let s_48_3: bool = ((s_48_1) < (s_48_2));
        // D s_48_4: not s_48_3
        let s_48_4: bool = !s_48_3;
        // N s_48_5: branch s_48_4 b56 b49
        if s_48_4 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #() : ()
        let s_49_0: () = ();
        // S s_49_1: call CurrentSecurityState(s_49_0)
        let s_49_1: u32 = CurrentSecurityState(state, tracer, s_49_0);
        // C s_49_2: const #3u : u32
        let s_49_2: u32 = 3;
        // S s_49_3: cmp-eq s_49_1 s_49_2
        let s_49_3: bool = ((s_49_1) == (s_49_2));
        // D s_49_4: write-var gs#31490 <= s_49_3
        fn_state.gs_31490 = s_49_3;
        // N s_49_5: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#31490:u8
        let s_50_0: bool = fn_state.gs_31490;
        // N s_50_1: branch s_50_0 b55 b51
        if s_50_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var mode:u8
        let s_51_0: u8 = fn_state.mode;
        // D s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 5u16);
        // C s_51_2: const #384u : u32
        let s_51_2: u32 = 384;
        // D s_51_3: read-reg s_51_2:u8
        let s_51_3: u8 = {
            let value = state.read_register::<u8>(s_51_2 as isize);
            tracer.read_register(s_51_2 as isize, value);
            value
        };
        // D s_51_4: cast zx s_51_3 -> bv
        let s_51_4: Bits = Bits::new(s_51_3 as u128, 5u16);
        // D s_51_5: cmp-eq s_51_1 s_51_4
        let s_51_5: bool = ((s_51_1) == (s_51_4));
        // D s_51_6: write-var gs#31491 <= s_51_5
        fn_state.gs_31491 = s_51_5;
        // N s_51_7: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#31491:u8
        let s_52_0: bool = fn_state.gs_31491;
        // N s_52_1: branch s_52_0 b54 b53
        if s_52_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_53_0: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_54_0: panic
        panic!("{:?}", ());
        // N s_54_1: return
        return;
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #1u : u8
        let s_55_0: bool = true;
        // D s_55_1: write-var gs#31491 <= s_55_0
        fn_state.gs_31491 = s_55_0;
        // N s_55_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #1u : u8
        let s_56_0: bool = true;
        // D s_56_1: write-var gs#31490 <= s_56_0
        fn_state.gs_31490 = s_56_0;
        // N s_56_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var SYSm:u8
        let s_57_0: u8 = fn_state.SYSm;
        // D s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 5u16);
        // C s_57_2: const #30u : u8
        let s_57_2: u8 = 30;
        // C s_57_3: cast zx s_57_2 -> bv
        let s_57_3: Bits = Bits::new(s_57_2 as u128, 5u16);
        // D s_57_4: cmp-eq s_57_1 s_57_3
        let s_57_4: bool = ((s_57_1) == (s_57_3));
        // D s_57_5: not s_57_4
        let s_57_5: bool = !s_57_4;
        // N s_57_6: branch s_57_5 b67 b58
        if s_57_5 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #432u : u32
        let s_58_0: u32 = 432;
        // D s_58_1: read-reg s_58_0:u8
        let s_58_1: u8 = {
            let value = state.read_register::<u8>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // C s_58_2: const #2u : u8
        let s_58_2: u8 = 2;
        // D s_58_3: cmp-lt s_58_1 s_58_2
        let s_58_3: bool = ((s_58_1) < (s_58_2));
        // D s_58_4: not s_58_3
        let s_58_4: bool = !s_58_3;
        // N s_58_5: branch s_58_4 b66 b59
        if s_58_4 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var mode:u8
        let s_59_0: u8 = fn_state.mode;
        // D s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 5u16);
        // C s_59_2: const #384u : u32
        let s_59_2: u32 = 384;
        // D s_59_3: read-reg s_59_2:u8
        let s_59_3: u8 = {
            let value = state.read_register::<u8>(s_59_2 as isize);
            tracer.read_register(s_59_2 as isize, value);
            value
        };
        // D s_59_4: cast zx s_59_3 -> bv
        let s_59_4: Bits = Bits::new(s_59_3 as u128, 5u16);
        // D s_59_5: cmp-eq s_59_1 s_59_4
        let s_59_5: bool = ((s_59_1) == (s_59_4));
        // N s_59_6: branch s_59_5 b65 b60
        if s_59_5 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var mode:u8
        let s_60_0: u8 = fn_state.mode;
        // D s_60_1: cast zx s_60_0 -> bv
        let s_60_1: Bits = Bits::new(s_60_0 as u128, 5u16);
        // C s_60_2: const #400u : u32
        let s_60_2: u32 = 400;
        // D s_60_3: read-reg s_60_2:u8
        let s_60_3: u8 = {
            let value = state.read_register::<u8>(s_60_2 as isize);
            tracer.read_register(s_60_2 as isize, value);
            value
        };
        // D s_60_4: cast zx s_60_3 -> bv
        let s_60_4: Bits = Bits::new(s_60_3 as u128, 5u16);
        // D s_60_5: cmp-eq s_60_1 s_60_4
        let s_60_5: bool = ((s_60_1) == (s_60_4));
        // D s_60_6: write-var gs#31494 <= s_60_5
        fn_state.gs_31494 = s_60_5;
        // N s_60_7: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#31494:u8
        let s_61_0: bool = fn_state.gs_31494;
        // D s_61_1: not s_61_0
        let s_61_1: bool = !s_61_0;
        // D s_61_2: write-var gs#31495 <= s_61_1
        fn_state.gs_31495 = s_61_1;
        // N s_61_3: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#31495:u8
        let s_62_0: bool = fn_state.gs_31495;
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
        // N s_63_0: return
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
        // C s_65_0: const #1u : u8
        let s_65_0: bool = true;
        // D s_65_1: write-var gs#31494 <= s_65_0
        fn_state.gs_31494 = s_65_0;
        // N s_65_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #1u : u8
        let s_66_0: bool = true;
        // D s_66_1: write-var gs#31495 <= s_66_0
        fn_state.gs_31495 = s_66_0;
        // N s_66_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var SYSm:u8
        let s_67_0: u8 = fn_state.SYSm;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 5u16);
        // C s_67_2: const #31u : u8
        let s_67_2: u8 = 31;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 5u16);
        // D s_67_4: cmp-eq s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) == (s_67_3));
        // D s_67_5: not s_67_4
        let s_67_5: bool = !s_67_4;
        // N s_67_6: branch s_67_5 b74 b68
        if s_67_5 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #432u : u32
        let s_68_0: u32 = 432;
        // D s_68_1: read-reg s_68_0:u8
        let s_68_1: u8 = {
            let value = state.read_register::<u8>(s_68_0 as isize);
            tracer.read_register(s_68_0 as isize, value);
            value
        };
        // C s_68_2: const #2u : u8
        let s_68_2: u8 = 2;
        // D s_68_3: cmp-lt s_68_1 s_68_2
        let s_68_3: bool = ((s_68_1) < (s_68_2));
        // D s_68_4: not s_68_3
        let s_68_4: bool = !s_68_3;
        // N s_68_5: branch s_68_4 b73 b69
        if s_68_4 {
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
        // D s_69_0: read-var mode:u8
        let s_69_0: u8 = fn_state.mode;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 5u16);
        // C s_69_2: const #384u : u32
        let s_69_2: u32 = 384;
        // D s_69_3: read-reg s_69_2:u8
        let s_69_3: u8 = {
            let value = state.read_register::<u8>(s_69_2 as isize);
            tracer.read_register(s_69_2 as isize, value);
            value
        };
        // D s_69_4: cast zx s_69_3 -> bv
        let s_69_4: Bits = Bits::new(s_69_3 as u128, 5u16);
        // D s_69_5: cmp-ne s_69_1 s_69_4
        let s_69_5: bool = ((s_69_1) != (s_69_4));
        // D s_69_6: write-var gs#31498 <= s_69_5
        fn_state.gs_31498 = s_69_5;
        // N s_69_7: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#31498:u8
        let s_70_0: bool = fn_state.gs_31498;
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
        // N s_71_0: return
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
        // C s_73_0: const #1u : u8
        let s_73_0: bool = true;
        // D s_73_1: write-var gs#31498 <= s_73_0
        fn_state.gs_31498 = s_73_0;
        // N s_73_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_74_0: panic
        panic!("{:?}", ());
        // N s_74_1: return
        return;
    }
}
