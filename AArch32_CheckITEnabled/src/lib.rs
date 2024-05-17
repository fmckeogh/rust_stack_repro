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
use u_get_HSCTLR_Type_ITD::*;
use SCTLR_read__2::*;
use SCTLR_read__1::*;
use ELUsingAArch32::*;
use u_get_SCTLRType_ITD::*;
use NextInstrAddr::*;
use u_get_SCTLR_Type_ITD::*;
use AArch32_MemSingle_read::*;
use CreateAccDescIFetch::*;
use HSCTLR_read::*;
use common::*;
pub fn AArch32_CheckITEnabled<T: Tracer>(state: &mut State, tracer: &T, mask: u8) -> () {
    #[derive(Default)]
    struct FunctionState {
        b__5: u16,
        gs_456617: Bits,
        gs_31926: bool,
        gs_31909: bool,
        b__4: u16,
        it_disabled: bool,
        gs_31935: bool,
        next_instrshadow_603: u16,
        gs_31934: bool,
        mask: u8,
    }
    let fn_state = FunctionState {
        mask,
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
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #432u : u32
        let s_0_3: u32 = 432;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-eq s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) == (s_0_5));
        // N s_0_7: branch s_0_6 b35 b1
        if s_0_6 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #440u : u32
        let s_1_0: u32 = 440;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: call ELUsingAArch32(s_1_1)
        let s_1_2: bool = ELUsingAArch32(state, tracer, s_1_1);
        // N s_1_3: branch s_1_2 b34 b2
        if s_1_2 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call SCTLR_read__1(s_2_0)
        let s_2_1: ProductType5c790c8ef59cc8b2 = SCTLR_read__1(state, tracer, s_2_0);
        // S s_2_2: call _get_SCTLRType_ITD(s_2_1)
        let s_2_2: bool = u_get_SCTLRType_ITD(state, tracer, s_2_1);
        // D s_2_3: write-var it_disabled <= s_2_2
        fn_state.it_disabled = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var it_disabled:u8
        let s_4_0: bool = fn_state.it_disabled;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 1u16);
        // C s_4_2: const #1u : u8
        let s_4_2: bool = true;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // N s_4_5: branch s_4_4 b6 b5
        if s_4_4 {
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
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var mask:u8
        let s_6_0: u8 = fn_state.mask;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 4u16);
        // C s_6_2: const #8u : u8
        let s_6_2: u8 = 8;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 4u16);
        // D s_6_4: cmp-ne s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) != (s_6_3));
        // N s_6_5: branch s_6_4 b33 b7
        if s_6_4 {
            return block_33(state, tracer, fn_state);
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
        // S s_7_1: call CreateAccDescIFetch(s_7_0)
        let s_7_1: ProductType9878976b5bcce9c9 = CreateAccDescIFetch(
            state,
            tracer,
            s_7_0,
        );
        // C s_7_2: const #1u : u8
        let s_7_2: bool = true;
        // C s_7_3: const #32s : i64
        let s_7_3: i64 = 32;
        // C s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // S s_7_5: call NextInstrAddr(s_7_4)
        let s_7_5: Bits = NextInstrAddr(state, tracer, s_7_4);
        // S s_7_6: cast reint s_7_5 -> u32
        let s_7_6: u32 = (s_7_5.value() as u32);
        // C s_7_7: const #2s : i64
        let s_7_7: i64 = 2;
        // S s_7_8: call AArch32_MemSingle_read(s_7_6, s_7_7, s_7_1, s_7_2)
        let s_7_8: Bits = AArch32_MemSingle_read(
            state,
            tracer,
            s_7_6,
            s_7_7,
            s_7_1,
            s_7_2,
        );
        // D s_7_9: write-var gs#456617 <= s_7_8
        fn_state.gs_456617 = s_7_8;
        // N s_7_10: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#456617:bv
        let s_8_0: Bits = fn_state.gs_456617;
        // D s_8_1: cast reint s_8_0 -> u16
        let s_8_1: u16 = (s_8_0.value() as u16);
        // D s_8_2: write-var next_instrshadow#603 <= s_8_1
        fn_state.next_instrshadow_603 = s_8_1;
        // D s_8_3: read-var next_instrshadow#603:u16
        let s_8_3: u16 = fn_state.next_instrshadow_603;
        // C s_8_4: const #14s : i
        let s_8_4: i128 = 14;
        // D s_8_5: cast zx s_8_3 -> bv
        let s_8_5: Bits = Bits::new(s_8_3 as u128, 16u16);
        // C s_8_6: const #1s : i64
        let s_8_6: i64 = 1;
        // C s_8_7: cast zx s_8_6 -> i
        let s_8_7: i128 = (i128::try_from(s_8_6).unwrap());
        // C s_8_8: const #1s : i
        let s_8_8: i128 = 1;
        // C s_8_9: add s_8_8 s_8_7
        let s_8_9: i128 = (s_8_8 + s_8_7);
        // D s_8_10: bit-extract s_8_5 s_8_4 s_8_9
        let s_8_10: Bits = (Bits::new(
            ((s_8_5) >> (s_8_4)).value(),
            u16::try_from(s_8_9).unwrap(),
        ));
        // D s_8_11: cast reint s_8_10 -> u8
        let s_8_11: u8 = (s_8_10.value() as u8);
        // D s_8_12: cast zx s_8_11 -> bv
        let s_8_12: Bits = Bits::new(s_8_11 as u128, 2u16);
        // C s_8_13: const #3u : u8
        let s_8_13: u8 = 3;
        // C s_8_14: cast zx s_8_13 -> bv
        let s_8_14: Bits = Bits::new(s_8_13 as u128, 2u16);
        // D s_8_15: cmp-eq s_8_12 s_8_14
        let s_8_15: bool = ((s_8_12) == (s_8_14));
        // D s_8_16: not s_8_15
        let s_8_16: bool = !s_8_15;
        // N s_8_17: branch s_8_16 b13 b9
        if s_8_16 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#31909 <= s_9_0
        fn_state.gs_31909 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#31909:u8
        let s_10_0: bool = fn_state.gs_31909;
        // N s_10_1: branch s_10_0 b12 b11
        if s_10_0 {
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
        // N s_11_0: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: panic
        panic!("{:?}", ());
        // N s_12_1: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var next_instrshadow#603:u16
        let s_13_0: u16 = fn_state.next_instrshadow_603;
        // C s_13_1: const #12s : i
        let s_13_1: i128 = 12;
        // D s_13_2: cast zx s_13_0 -> bv
        let s_13_2: Bits = Bits::new(s_13_0 as u128, 16u16);
        // C s_13_3: const #1s : i64
        let s_13_3: i64 = 1;
        // C s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // C s_13_5: const #3s : i
        let s_13_5: i128 = 3;
        // C s_13_6: add s_13_5 s_13_4
        let s_13_6: i128 = (s_13_5 + s_13_4);
        // D s_13_7: bit-extract s_13_2 s_13_1 s_13_6
        let s_13_7: Bits = (Bits::new(
            ((s_13_2) >> (s_13_1)).value(),
            u16::try_from(s_13_6).unwrap(),
        ));
        // D s_13_8: cast reint s_13_7 -> u8
        let s_13_8: u8 = (s_13_7.value() as u8);
        // D s_13_9: cast zx s_13_8 -> bv
        let s_13_9: Bits = Bits::new(s_13_8 as u128, 4u16);
        // C s_13_10: const #11u : u8
        let s_13_10: u8 = 11;
        // C s_13_11: cast zx s_13_10 -> bv
        let s_13_11: Bits = Bits::new(s_13_10 as u128, 4u16);
        // D s_13_12: cmp-eq s_13_9 s_13_11
        let s_13_12: bool = ((s_13_9) == (s_13_11));
        // D s_13_13: not s_13_12
        let s_13_13: bool = !s_13_12;
        // N s_13_14: branch s_13_13 b15 b14
        if s_13_13 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#31909 <= s_14_0
        fn_state.gs_31909 = s_14_0;
        // N s_14_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var next_instrshadow#603:u16
        let s_15_0: u16 = fn_state.next_instrshadow_603;
        // C s_15_1: const #11s : i
        let s_15_1: i128 = 11;
        // D s_15_2: cast zx s_15_0 -> bv
        let s_15_2: Bits = Bits::new(s_15_0 as u128, 16u16);
        // C s_15_3: const #1s : i64
        let s_15_3: i64 = 1;
        // C s_15_4: cast zx s_15_3 -> i
        let s_15_4: i128 = (i128::try_from(s_15_3).unwrap());
        // C s_15_5: const #4s : i
        let s_15_5: i128 = 4;
        // C s_15_6: add s_15_5 s_15_4
        let s_15_6: i128 = (s_15_5 + s_15_4);
        // D s_15_7: bit-extract s_15_2 s_15_1 s_15_6
        let s_15_7: Bits = (Bits::new(
            ((s_15_2) >> (s_15_1)).value(),
            u16::try_from(s_15_6).unwrap(),
        ));
        // D s_15_8: cast reint s_15_7 -> u8
        let s_15_8: u8 = (s_15_7.value() as u8);
        // D s_15_9: cast zx s_15_8 -> bv
        let s_15_9: Bits = Bits::new(s_15_8 as u128, 5u16);
        // C s_15_10: const #20u : u8
        let s_15_10: u8 = 20;
        // C s_15_11: cast zx s_15_10 -> bv
        let s_15_11: Bits = Bits::new(s_15_10 as u128, 5u16);
        // D s_15_12: cmp-eq s_15_9 s_15_11
        let s_15_12: bool = ((s_15_9) == (s_15_11));
        // D s_15_13: not s_15_12
        let s_15_13: bool = !s_15_12;
        // N s_15_14: branch s_15_13 b17 b16
        if s_15_13 {
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
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#31909 <= s_16_0
        fn_state.gs_31909 = s_16_0;
        // N s_16_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var next_instrshadow#603:u16
        let s_17_0: u16 = fn_state.next_instrshadow_603;
        // C s_17_1: const #11s : i
        let s_17_1: i128 = 11;
        // D s_17_2: cast zx s_17_0 -> bv
        let s_17_2: Bits = Bits::new(s_17_0 as u128, 16u16);
        // C s_17_3: const #1s : i64
        let s_17_3: i64 = 1;
        // C s_17_4: cast zx s_17_3 -> i
        let s_17_4: i128 = (i128::try_from(s_17_3).unwrap());
        // C s_17_5: const #4s : i
        let s_17_5: i128 = 4;
        // C s_17_6: add s_17_5 s_17_4
        let s_17_6: i128 = (s_17_5 + s_17_4);
        // D s_17_7: bit-extract s_17_2 s_17_1 s_17_6
        let s_17_7: Bits = (Bits::new(
            ((s_17_2) >> (s_17_1)).value(),
            u16::try_from(s_17_6).unwrap(),
        ));
        // D s_17_8: cast reint s_17_7 -> u8
        let s_17_8: u8 = (s_17_7.value() as u8);
        // D s_17_9: cast zx s_17_8 -> bv
        let s_17_9: Bits = Bits::new(s_17_8 as u128, 5u16);
        // C s_17_10: const #9u : u8
        let s_17_10: u8 = 9;
        // C s_17_11: cast zx s_17_10 -> bv
        let s_17_11: Bits = Bits::new(s_17_10 as u128, 5u16);
        // D s_17_12: cmp-eq s_17_9 s_17_11
        let s_17_12: bool = ((s_17_9) == (s_17_11));
        // D s_17_13: not s_17_12
        let s_17_13: bool = !s_17_12;
        // N s_17_14: branch s_17_13 b19 b18
        if s_17_13 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#31909 <= s_18_0
        fn_state.gs_31909 = s_18_0;
        // N s_18_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var next_instrshadow#603:u16
        let s_19_0: u16 = fn_state.next_instrshadow_603;
        // D s_19_1: write-var b__4 <= s_19_0
        fn_state.b__4 = s_19_0;
        // C s_19_2: const #10s : i
        let s_19_2: i128 = 10;
        // D s_19_3: read-var b__4:u16
        let s_19_3: u16 = fn_state.b__4;
        // D s_19_4: cast zx s_19_3 -> bv
        let s_19_4: Bits = Bits::new(s_19_3 as u128, 16u16);
        // C s_19_5: const #1s : i64
        let s_19_5: i64 = 1;
        // C s_19_6: cast zx s_19_5 -> i
        let s_19_6: i128 = (i128::try_from(s_19_5).unwrap());
        // C s_19_7: const #5s : i
        let s_19_7: i128 = 5;
        // C s_19_8: add s_19_7 s_19_6
        let s_19_8: i128 = (s_19_7 + s_19_6);
        // D s_19_9: bit-extract s_19_4 s_19_2 s_19_8
        let s_19_9: Bits = (Bits::new(
            ((s_19_4) >> (s_19_2)).value(),
            u16::try_from(s_19_8).unwrap(),
        ));
        // D s_19_10: cast reint s_19_9 -> u8
        let s_19_10: u8 = (s_19_9.value() as u8);
        // D s_19_11: cast zx s_19_10 -> bv
        let s_19_11: Bits = Bits::new(s_19_10 as u128, 6u16);
        // C s_19_12: const #17u : u8
        let s_19_12: u8 = 17;
        // C s_19_13: cast zx s_19_12 -> bv
        let s_19_13: Bits = Bits::new(s_19_12 as u128, 6u16);
        // D s_19_14: cmp-eq s_19_11 s_19_13
        let s_19_14: bool = ((s_19_11) == (s_19_13));
        // N s_19_15: branch s_19_14 b32 b20
        if s_19_14 {
            return block_32(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#31926 <= s_20_0
        fn_state.gs_31926 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#31926:u8
        let s_21_0: bool = fn_state.gs_31926;
        // D s_21_1: not s_21_0
        let s_21_1: bool = !s_21_0;
        // N s_21_2: branch s_21_1 b23 b22
        if s_21_1 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#31909 <= s_22_0
        fn_state.gs_31909 = s_22_0;
        // N s_22_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var next_instrshadow#603:u16
        let s_23_0: u16 = fn_state.next_instrshadow_603;
        // D s_23_1: write-var b__5 <= s_23_0
        fn_state.b__5 = s_23_0;
        // C s_23_2: const #10s : i
        let s_23_2: i128 = 10;
        // D s_23_3: read-var b__5:u16
        let s_23_3: u16 = fn_state.b__5;
        // D s_23_4: cast zx s_23_3 -> bv
        let s_23_4: Bits = Bits::new(s_23_3 as u128, 16u16);
        // C s_23_5: const #1s : i64
        let s_23_5: i64 = 1;
        // C s_23_6: cast zx s_23_5 -> i
        let s_23_6: i128 = (i128::try_from(s_23_5).unwrap());
        // C s_23_7: const #5s : i
        let s_23_7: i128 = 5;
        // C s_23_8: add s_23_7 s_23_6
        let s_23_8: i128 = (s_23_7 + s_23_6);
        // D s_23_9: bit-extract s_23_4 s_23_2 s_23_8
        let s_23_9: Bits = (Bits::new(
            ((s_23_4) >> (s_23_2)).value(),
            u16::try_from(s_23_8).unwrap(),
        ));
        // D s_23_10: cast reint s_23_9 -> u8
        let s_23_10: u8 = (s_23_9.value() as u8);
        // D s_23_11: cast zx s_23_10 -> bv
        let s_23_11: Bits = Bits::new(s_23_10 as u128, 6u16);
        // C s_23_12: const #17u : u8
        let s_23_12: u8 = 17;
        // C s_23_13: cast zx s_23_12 -> bv
        let s_23_13: Bits = Bits::new(s_23_12 as u128, 6u16);
        // D s_23_14: cmp-eq s_23_11 s_23_13
        let s_23_14: bool = ((s_23_11) == (s_23_13));
        // N s_23_15: branch s_23_14 b28 b24
        if s_23_14 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var gs#31935 <= s_24_0
        fn_state.gs_31935 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#31935:u8
        let s_25_0: bool = fn_state.gs_31935;
        // D s_25_1: not s_25_0
        let s_25_1: bool = !s_25_0;
        // N s_25_2: branch s_25_1 b27 b26
        if s_25_1 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#31909 <= s_26_0
        fn_state.gs_31909 = s_26_0;
        // N s_26_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#31909 <= s_27_0
        fn_state.gs_31909 = s_27_0;
        // N s_27_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #7s : i
        let s_28_0: i128 = 7;
        // D s_28_1: read-var b__5:u16
        let s_28_1: u16 = fn_state.b__5;
        // D s_28_2: cast zx s_28_1 -> bv
        let s_28_2: Bits = Bits::new(s_28_1 as u128, 16u16);
        // C s_28_3: const #1s : i64
        let s_28_3: i64 = 1;
        // C s_28_4: cast zx s_28_3 -> i
        let s_28_4: i128 = (i128::try_from(s_28_3).unwrap());
        // C s_28_5: const #0s : i
        let s_28_5: i128 = 0;
        // C s_28_6: add s_28_5 s_28_4
        let s_28_6: i128 = (s_28_5 + s_28_4);
        // D s_28_7: bit-extract s_28_2 s_28_0 s_28_6
        let s_28_7: Bits = (Bits::new(
            ((s_28_2) >> (s_28_0)).value(),
            u16::try_from(s_28_6).unwrap(),
        ));
        // D s_28_8: cast reint s_28_7 -> u8
        let s_28_8: bool = ((s_28_7.value()) != 0);
        // D s_28_9: cast zx s_28_8 -> bv
        let s_28_9: Bits = Bits::new(s_28_8 as u128, 1u16);
        // C s_28_10: const #1u : u8
        let s_28_10: bool = true;
        // C s_28_11: cast zx s_28_10 -> bv
        let s_28_11: Bits = Bits::new(s_28_10 as u128, 1u16);
        // D s_28_12: cmp-eq s_28_9 s_28_11
        let s_28_12: bool = ((s_28_9) == (s_28_11));
        // N s_28_13: branch s_28_12 b31 b29
        if s_28_12 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#31934 <= s_29_0
        fn_state.gs_31934 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#31934:u8
        let s_30_0: bool = fn_state.gs_31934;
        // D s_30_1: write-var gs#31935 <= s_30_0
        fn_state.gs_31935 = s_30_0;
        // N s_30_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #0s : i
        let s_31_0: i128 = 0;
        // D s_31_1: read-var b__5:u16
        let s_31_1: u16 = fn_state.b__5;
        // D s_31_2: cast zx s_31_1 -> bv
        let s_31_2: Bits = Bits::new(s_31_1 as u128, 16u16);
        // C s_31_3: const #1s : i64
        let s_31_3: i64 = 1;
        // C s_31_4: cast zx s_31_3 -> i
        let s_31_4: i128 = (i128::try_from(s_31_3).unwrap());
        // C s_31_5: const #2s : i
        let s_31_5: i128 = 2;
        // C s_31_6: add s_31_5 s_31_4
        let s_31_6: i128 = (s_31_5 + s_31_4);
        // D s_31_7: bit-extract s_31_2 s_31_0 s_31_6
        let s_31_7: Bits = (Bits::new(
            ((s_31_2) >> (s_31_0)).value(),
            u16::try_from(s_31_6).unwrap(),
        ));
        // D s_31_8: cast reint s_31_7 -> u8
        let s_31_8: u8 = (s_31_7.value() as u8);
        // D s_31_9: cast zx s_31_8 -> bv
        let s_31_9: Bits = Bits::new(s_31_8 as u128, 3u16);
        // C s_31_10: const #7u : u8
        let s_31_10: u8 = 7;
        // C s_31_11: cast zx s_31_10 -> bv
        let s_31_11: Bits = Bits::new(s_31_10 as u128, 3u16);
        // D s_31_12: cmp-eq s_31_9 s_31_11
        let s_31_12: bool = ((s_31_9) == (s_31_11));
        // D s_31_13: write-var gs#31934 <= s_31_12
        fn_state.gs_31934 = s_31_12;
        // N s_31_14: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #3s : i
        let s_32_0: i128 = 3;
        // D s_32_1: read-var b__4:u16
        let s_32_1: u16 = fn_state.b__4;
        // D s_32_2: cast zx s_32_1 -> bv
        let s_32_2: Bits = Bits::new(s_32_1 as u128, 16u16);
        // C s_32_3: const #1s : i64
        let s_32_3: i64 = 1;
        // C s_32_4: cast zx s_32_3 -> i
        let s_32_4: i128 = (i128::try_from(s_32_3).unwrap());
        // C s_32_5: const #3s : i
        let s_32_5: i128 = 3;
        // C s_32_6: add s_32_5 s_32_4
        let s_32_6: i128 = (s_32_5 + s_32_4);
        // D s_32_7: bit-extract s_32_2 s_32_0 s_32_6
        let s_32_7: Bits = (Bits::new(
            ((s_32_2) >> (s_32_0)).value(),
            u16::try_from(s_32_6).unwrap(),
        ));
        // D s_32_8: cast reint s_32_7 -> u8
        let s_32_8: u8 = (s_32_7.value() as u8);
        // D s_32_9: cast zx s_32_8 -> bv
        let s_32_9: Bits = Bits::new(s_32_8 as u128, 4u16);
        // C s_32_10: const #15u : u8
        let s_32_10: u8 = 15;
        // C s_32_11: cast zx s_32_10 -> bv
        let s_32_11: Bits = Bits::new(s_32_10 as u128, 4u16);
        // D s_32_12: cmp-eq s_32_9 s_32_11
        let s_32_12: bool = ((s_32_9) == (s_32_11));
        // D s_32_13: write-var gs#31926 <= s_32_12
        fn_state.gs_31926 = s_32_12;
        // N s_32_14: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_33_0: panic
        panic!("{:?}", ());
        // N s_33_1: return
        return;
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #() : ()
        let s_34_0: () = ();
        // S s_34_1: call SCTLR_read__2(s_34_0)
        let s_34_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_34_0);
        // S s_34_2: call _get_SCTLR_Type_ITD(s_34_1)
        let s_34_2: bool = u_get_SCTLR_Type_ITD(state, tracer, s_34_1);
        // D s_34_3: write-var it_disabled <= s_34_2
        fn_state.it_disabled = s_34_2;
        // N s_34_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #() : ()
        let s_35_0: () = ();
        // S s_35_1: call HSCTLR_read(s_35_0)
        let s_35_1: ProductType700c18a878c5601b = HSCTLR_read(state, tracer, s_35_0);
        // S s_35_2: call _get_HSCTLR_Type_ITD(s_35_1)
        let s_35_2: bool = u_get_HSCTLR_Type_ITD(state, tracer, s_35_1);
        // D s_35_3: write-var it_disabled <= s_35_2
        fn_state.it_disabled = s_35_2;
        // N s_35_4: jump b4
        return block_4(state, tracer, fn_state);
    }
}
