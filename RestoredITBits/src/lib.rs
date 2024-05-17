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
use HSCTLR_read::*;
use SCTLR_read__2::*;
use u_get_HSCTLR_Type_ITD::*;
use IsZero::*;
use ConstrainUnpredictableBool::*;
use u_get_SCTLR_Type_ITD::*;
use common::*;
pub fn RestoredITBits<T: Tracer>(state: &mut State, tracer: &T, spsr: Bits) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        gs_5525: bool,
        it: u8,
        gs_5520: bool,
        return_value: u8,
        itd: bool,
        gs_5529: bool,
        gs_5528: bool,
        spsr: Bits,
    }
    let fn_state = FunctionState {
        spsr,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_0_0: const #10s : i
        let s_0_0: i128 = 10;
        // D s_0_1: read-var spsr:bv
        let s_0_1: Bits = fn_state.spsr;
        // C s_0_2: const #1s : i64
        let s_0_2: i64 = 1;
        // C s_0_3: cast zx s_0_2 -> i
        let s_0_3: i128 = (i128::try_from(s_0_2).unwrap());
        // C s_0_4: const #5s : i
        let s_0_4: i128 = 5;
        // C s_0_5: add s_0_4 s_0_3
        let s_0_5: i128 = (s_0_4 + s_0_3);
        // D s_0_6: bit-extract s_0_1 s_0_0 s_0_5
        let s_0_6: Bits = (Bits::new(
            ((s_0_1) >> (s_0_0)).value(),
            u16::try_from(s_0_5).unwrap(),
        ));
        // D s_0_7: cast reint s_0_6 -> u8
        let s_0_7: u8 = (s_0_6.value() as u8);
        // C s_0_8: const #25s : i
        let s_0_8: i128 = 25;
        // D s_0_9: read-var spsr:bv
        let s_0_9: Bits = fn_state.spsr;
        // C s_0_10: const #1s : i64
        let s_0_10: i64 = 1;
        // C s_0_11: cast zx s_0_10 -> i
        let s_0_11: i128 = (i128::try_from(s_0_10).unwrap());
        // C s_0_12: const #1s : i
        let s_0_12: i128 = 1;
        // C s_0_13: add s_0_12 s_0_11
        let s_0_13: i128 = (s_0_12 + s_0_11);
        // D s_0_14: bit-extract s_0_9 s_0_8 s_0_13
        let s_0_14: Bits = (Bits::new(
            ((s_0_9) >> (s_0_8)).value(),
            u16::try_from(s_0_13).unwrap(),
        ));
        // D s_0_15: cast reint s_0_14 -> u8
        let s_0_15: u8 = (s_0_14.value() as u8);
        // D s_0_16: cast zx s_0_7 -> bv
        let s_0_16: Bits = Bits::new(s_0_7 as u128, 6u16);
        // D s_0_17: cast zx s_0_15 -> bv
        let s_0_17: Bits = Bits::new(s_0_15 as u128, 2u16);
        // D s_0_18: cast reint s_0_16 -> u128
        let s_0_18: u128 = (s_0_16.value() as u128);
        // D s_0_19: size-of s_0_16
        let s_0_19: u16 = s_0_16.length();
        // D s_0_20: cast reint s_0_17 -> u128
        let s_0_20: u128 = (s_0_17.value() as u128);
        // D s_0_21: size-of s_0_17
        let s_0_21: u16 = s_0_17.length();
        // D s_0_22: lsl s_0_18 s_0_21
        let s_0_22: u128 = s_0_18 << s_0_21;
        // D s_0_23: or s_0_22 s_0_20
        let s_0_23: u128 = ((s_0_22) | (s_0_20));
        // D s_0_24: add s_0_19 s_0_21
        let s_0_24: u16 = (s_0_19 + s_0_21);
        // D s_0_25: create-bits s_0_23 s_0_24
        let s_0_25: Bits = Bits::new(s_0_23, s_0_24);
        // D s_0_26: cast reint s_0_25 -> u8
        let s_0_26: u8 = (s_0_25.value() as u8);
        // D s_0_27: write-var it <= s_0_26
        fn_state.it = s_0_26;
        // C s_0_28: const #16980u : u32
        let s_0_28: u32 = 16980;
        // D s_0_29: read-reg s_0_28:u8
        let s_0_29: bool = {
            let value = state.read_register::<bool>(s_0_28 as isize);
            tracer.read_register(s_0_28 as isize, value);
            value
        };
        // D s_0_30: cast zx s_0_29 -> bv
        let s_0_30: Bits = Bits::new(s_0_29 as u128, 1u16);
        // C s_0_31: const #1u : u8
        let s_0_31: bool = true;
        // C s_0_32: cast zx s_0_31 -> bv
        let s_0_32: Bits = Bits::new(s_0_31 as u128, 1u16);
        // D s_0_33: cmp-eq s_0_30 s_0_32
        let s_0_33: bool = ((s_0_30) == (s_0_32));
        // N s_0_34: branch s_0_33 b22 b1
        if s_0_33 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_1_0: const #4s : i
        let s_1_0: i128 = 4;
        // D s_1_1: read-var it:u8
        let s_1_1: u8 = fn_state.it;
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 8u16);
        // C s_1_3: const #1s : i64
        let s_1_3: i64 = 1;
        // C s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // C s_1_5: const #3s : i
        let s_1_5: i128 = 3;
        // C s_1_6: add s_1_5 s_1_4
        let s_1_6: i128 = (s_1_5 + s_1_4);
        // D s_1_7: bit-extract s_1_2 s_1_0 s_1_6
        let s_1_7: Bits = (Bits::new(
            ((s_1_2) >> (s_1_0)).value(),
            u16::try_from(s_1_6).unwrap(),
        ));
        // D s_1_8: cast reint s_1_7 -> u8
        let s_1_8: u8 = (s_1_7.value() as u8);
        // D s_1_9: cast zx s_1_8 -> bv
        let s_1_9: Bits = Bits::new(s_1_8 as u128, 4u16);
        // D s_1_10: call IsZero(s_1_9)
        let s_1_10: bool = IsZero(state, tracer, s_1_9);
        // D s_1_11: not s_1_10
        let s_1_11: bool = !s_1_10;
        // N s_1_12: branch s_1_11 b21 b2
        if s_1_11 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#5520 <= s_2_0
        fn_state.gs_5520 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_3_0: read-var gs#5520:u8
        let s_3_0: bool = fn_state.gs_5520;
        // N s_3_1: branch s_3_0 b20 b4
        if s_3_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_4_0: const #16975u : u32
        let s_4_0: u32 = 16975;
        // D s_4_1: read-reg s_4_0:u8
        let s_4_1: u8 = {
            let value = state.read_register::<u8>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 2u16);
        // C s_4_3: const #432u : u32
        let s_4_3: u32 = 432;
        // D s_4_4: read-reg s_4_3:u8
        let s_4_4: u8 = {
            let value = state.read_register::<u8>(s_4_3 as isize);
            tracer.read_register(s_4_3 as isize, value);
            value
        };
        // D s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 2u16);
        // D s_4_6: cmp-eq s_4_2 s_4_5
        let s_4_6: bool = ((s_4_2) == (s_4_5));
        // N s_4_7: branch s_4_6 b19 b5
        if s_4_6 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call SCTLR_read__2(s_5_0)
        let s_5_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_5_0);
        // S s_5_2: call _get_SCTLR_Type_ITD(s_5_1)
        let s_5_2: bool = u_get_SCTLR_Type_ITD(state, tracer, s_5_1);
        // D s_5_3: write-var itd <= s_5_2
        fn_state.itd = s_5_2;
        // N s_5_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_6_0: const #5s : i
        let s_6_0: i128 = 5;
        // D s_6_1: read-var spsr:bv
        let s_6_1: Bits = fn_state.spsr;
        // C s_6_2: const #1u : u64
        let s_6_2: u64 = 1;
        // D s_6_3: bit-extract s_6_1 s_6_0 s_6_2
        let s_6_3: Bits = (Bits::new(
            ((s_6_1) >> (s_6_0)).value(),
            u16::try_from(s_6_2).unwrap(),
        ));
        // D s_6_4: cast reint s_6_3 -> u8
        let s_6_4: bool = ((s_6_3.value()) != 0);
        // C s_6_5: const #0s : i
        let s_6_5: i128 = 0;
        // C s_6_6: const #0u : u64
        let s_6_6: u64 = 0;
        // D s_6_7: cast zx s_6_4 -> u64
        let s_6_7: u64 = (s_6_4 as u64);
        // C s_6_8: const #1u : u64
        let s_6_8: u64 = 1;
        // D s_6_9: and s_6_7 s_6_8
        let s_6_9: u64 = ((s_6_7) & (s_6_8));
        // D s_6_10: cmp-eq s_6_9 s_6_8
        let s_6_10: bool = ((s_6_9) == (s_6_8));
        // D s_6_11: lsl s_6_7 s_6_5
        let s_6_11: u64 = s_6_7 << s_6_5;
        // D s_6_12: or s_6_6 s_6_11
        let s_6_12: u64 = ((s_6_6) | (s_6_11));
        // D s_6_13: cmpl s_6_11
        let s_6_13: u64 = !s_6_11;
        // D s_6_14: and s_6_6 s_6_13
        let s_6_14: u64 = ((s_6_6) & (s_6_13));
        // D s_6_15: select s_6_10 s_6_12 s_6_14
        let s_6_15: u64 = if s_6_10 { s_6_12 } else { s_6_14 };
        // D s_6_16: cast trunc s_6_15 -> u8
        let s_6_16: bool = ((s_6_15) != 0);
        // D s_6_17: cast zx s_6_16 -> bv
        let s_6_17: Bits = Bits::new(s_6_16 as u128, 1u16);
        // C s_6_18: const #0u : u8
        let s_6_18: bool = false;
        // C s_6_19: cast zx s_6_18 -> bv
        let s_6_19: Bits = Bits::new(s_6_18 as u128, 1u16);
        // D s_6_20: cmp-eq s_6_17 s_6_19
        let s_6_20: bool = ((s_6_17) == (s_6_19));
        // N s_6_21: branch s_6_20 b18 b7
        if s_6_20 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#5525 <= s_7_0
        fn_state.gs_5525 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_8_0: read-var gs#5525:u8
        let s_8_0: bool = fn_state.gs_5525;
        // N s_8_1: branch s_8_0 b17 b9
        if s_8_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_9_0: read-var itd:u8
        let s_9_0: bool = fn_state.itd;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 1u16);
        // C s_9_2: const #1u : u8
        let s_9_2: bool = true;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // N s_9_5: branch s_9_4 b16 b10
        if s_9_4 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#5528 <= s_10_0
        fn_state.gs_5528 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_11_0: read-var gs#5528:u8
        let s_11_0: bool = fn_state.gs_5528;
        // D s_11_1: write-var gs#5529 <= s_11_0
        fn_state.gs_5529 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_12_0: read-var gs#5529:u8
        let s_12_0: bool = fn_state.gs_5529;
        // N s_12_1: branch s_12_0 b15 b13
        if s_12_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_13_0: read-var it:u8
        let s_13_0: u8 = fn_state.it;
        // D s_13_1: write-var return_value <= s_13_0
        fn_state.return_value = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_14_0: read-var return_value:u8
        let s_14_0: u8 = fn_state.return_value;
        // N s_14_1: return s_14_0
        return s_14_0;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_15_0: const #0u : u8
        let s_15_0: u8 = 0;
        // D s_15_1: write-var return_value <= s_15_0
        fn_state.return_value = s_15_0;
        // N s_15_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_16_0: const #0s : i
        let s_16_0: i128 = 0;
        // D s_16_1: read-var it:u8
        let s_16_1: u8 = fn_state.it;
        // D s_16_2: cast zx s_16_1 -> bv
        let s_16_2: Bits = Bits::new(s_16_1 as u128, 8u16);
        // C s_16_3: const #1s : i64
        let s_16_3: i64 = 1;
        // C s_16_4: cast zx s_16_3 -> i
        let s_16_4: i128 = (i128::try_from(s_16_3).unwrap());
        // C s_16_5: const #2s : i
        let s_16_5: i128 = 2;
        // C s_16_6: add s_16_5 s_16_4
        let s_16_6: i128 = (s_16_5 + s_16_4);
        // D s_16_7: bit-extract s_16_2 s_16_0 s_16_6
        let s_16_7: Bits = (Bits::new(
            ((s_16_2) >> (s_16_0)).value(),
            u16::try_from(s_16_6).unwrap(),
        ));
        // D s_16_8: cast reint s_16_7 -> u8
        let s_16_8: u8 = (s_16_7.value() as u8);
        // D s_16_9: cast zx s_16_8 -> bv
        let s_16_9: Bits = Bits::new(s_16_8 as u128, 3u16);
        // D s_16_10: call IsZero(s_16_9)
        let s_16_10: bool = IsZero(state, tracer, s_16_9);
        // D s_16_11: not s_16_10
        let s_16_11: bool = !s_16_10;
        // D s_16_12: write-var gs#5528 <= s_16_11
        fn_state.gs_5528 = s_16_11;
        // N s_16_13: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#5529 <= s_17_0
        fn_state.gs_5529 = s_17_0;
        // N s_17_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_18_0: read-var it:u8
        let s_18_0: u8 = fn_state.it;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 8u16);
        // D s_18_2: call IsZero(s_18_1)
        let s_18_2: bool = IsZero(state, tracer, s_18_1);
        // D s_18_3: not s_18_2
        let s_18_3: bool = !s_18_2;
        // D s_18_4: write-var gs#5525 <= s_18_3
        fn_state.gs_5525 = s_18_3;
        // N s_18_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call HSCTLR_read(s_19_0)
        let s_19_1: ProductType700c18a878c5601b = HSCTLR_read(state, tracer, s_19_0);
        // S s_19_2: call _get_HSCTLR_Type_ITD(s_19_1)
        let s_19_2: bool = u_get_HSCTLR_Type_ITD(state, tracer, s_19_1);
        // D s_19_3: write-var itd <= s_19_2
        fn_state.itd = s_19_2;
        // N s_19_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_20_0: const #0u : u8
        let s_20_0: u8 = 0;
        // D s_20_1: write-var return_value <= s_20_0
        fn_state.return_value = s_20_0;
        // N s_20_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_21_0: const #0s : i
        let s_21_0: i128 = 0;
        // D s_21_1: read-var it:u8
        let s_21_1: u8 = fn_state.it;
        // D s_21_2: cast zx s_21_1 -> bv
        let s_21_2: Bits = Bits::new(s_21_1 as u128, 8u16);
        // C s_21_3: const #1s : i64
        let s_21_3: i64 = 1;
        // C s_21_4: cast zx s_21_3 -> i
        let s_21_4: i128 = (i128::try_from(s_21_3).unwrap());
        // C s_21_5: const #3s : i
        let s_21_5: i128 = 3;
        // C s_21_6: add s_21_5 s_21_4
        let s_21_6: i128 = (s_21_5 + s_21_4);
        // D s_21_7: bit-extract s_21_2 s_21_0 s_21_6
        let s_21_7: Bits = (Bits::new(
            ((s_21_2) >> (s_21_0)).value(),
            u16::try_from(s_21_6).unwrap(),
        ));
        // D s_21_8: cast reint s_21_7 -> u8
        let s_21_8: u8 = (s_21_7.value() as u8);
        // D s_21_9: cast zx s_21_8 -> bv
        let s_21_9: Bits = Bits::new(s_21_8 as u128, 4u16);
        // D s_21_10: call IsZero(s_21_9)
        let s_21_10: bool = IsZero(state, tracer, s_21_9);
        // D s_21_11: write-var gs#5520 <= s_21_10
        fn_state.gs_5520 = s_21_10;
        // N s_21_12: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_22_0: const #22u : u32
        let s_22_0: u32 = 22;
        // S s_22_1: call ConstrainUnpredictableBool(s_22_0)
        let s_22_1: bool = ConstrainUnpredictableBool(state, tracer, s_22_0);
        // N s_22_2: branch s_22_1 b24 b23
        if s_22_1 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_23_0: read-var it:u8
        let s_23_0: u8 = fn_state.it;
        // D s_23_1: write-var return_value <= s_23_0
        fn_state.return_value = s_23_0;
        // N s_23_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_24_0: const #0u : u8
        let s_24_0: u8 = 0;
        // D s_24_1: write-var return_value <= s_24_0
        fn_state.return_value = s_24_0;
        // N s_24_2: jump b14
        return block_14(state, tracer, fn_state);
    }
}
