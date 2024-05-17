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
use TargetSecurityState::*;
use RESTRICT_PREDICTIONS::*;
use VMID_read::*;
use EL2Enabled::*;
use ASID_read::*;
use EndOfInstruction::*;
use u__UNKNOWN_bit::*;
use common::*;
pub fn AArch32_RestrictPrediction<T: Tracer>(
    state: &mut State,
    tracer: &T,
    val_name: u32,
    restriction: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_30475: bool,
        gs_30492: bool,
        c: ProductType63dc1b957c45bf6b,
        target_el: u8,
        gs_30491: bool,
        val_name: u32,
        restriction: u32,
    }
    let fn_state = FunctionState {
        val_name,
        restriction,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #24s : i
        let s_0_0: i128 = 24;
        // D s_0_1: read-var val_name:u32
        let s_0_1: u32 = fn_state.val_name;
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 32u16);
        // C s_0_3: const #1s : i64
        let s_0_3: i64 = 1;
        // C s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // C s_0_5: const #1s : i
        let s_0_5: i128 = 1;
        // C s_0_6: add s_0_5 s_0_4
        let s_0_6: i128 = (s_0_5 + s_0_4);
        // D s_0_7: bit-extract s_0_2 s_0_0 s_0_6
        let s_0_7: Bits = (Bits::new(
            ((s_0_2) >> (s_0_0)).value(),
            u16::try_from(s_0_6).unwrap(),
        ));
        // D s_0_8: cast reint s_0_7 -> u8
        let s_0_8: u8 = (s_0_7.value() as u8);
        // D s_0_9: write-var target_el <= s_0_8
        fn_state.target_el = s_0_8;
        // D s_0_10: read-var target_el:u8
        let s_0_10: u8 = fn_state.target_el;
        // C s_0_11: const #2u : u8
        let s_0_11: u8 = 2;
        // D s_0_12: cmp-lt s_0_10 s_0_11
        let s_0_12: bool = ((s_0_10) < (s_0_11));
        // D s_0_13: not s_0_12
        let s_0_13: bool = !s_0_12;
        // N s_0_14: branch s_0_13 b24 b1
        if s_0_13 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var target_el:u8
        let s_1_0: u8 = fn_state.target_el;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // C s_1_4: const #16975u : u32
        let s_1_4: u32 = 16975;
        // D s_1_5: read-reg s_1_4:u8
        let s_1_5: u8 = {
            let value = state.read_register::<u8>(s_1_4 as isize);
            tracer.read_register(s_1_4 as isize, value);
            value
        };
        // D s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 2u16);
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (s_1_6.value() as i128);
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: cast zx s_1_3 -> i
        let s_1_9: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_10: cast zx s_1_8 -> i
        let s_1_10: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_11: cmp-gt s_1_9 s_1_10
        let s_1_11: bool = ((s_1_9) > (s_1_10));
        // D s_1_12: write-var gs#30475 <= s_1_11
        fn_state.gs_30475 = s_1_11;
        // N s_1_13: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#30475:u8
        let s_2_0: bool = fn_state.gs_30475;
        // N s_2_1: branch s_2_0 b23 b3
        if s_2_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
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
        // C s_4_0: const #26s : i
        let s_4_0: i128 = 26;
        // D s_4_1: read-var val_name:u32
        let s_4_1: u32 = fn_state.val_name;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 32u16);
        // C s_4_3: const #1u : u64
        let s_4_3: u64 = 1;
        // D s_4_4: bit-extract s_4_2 s_4_0 s_4_3
        let s_4_4: Bits = (Bits::new(
            ((s_4_2) >> (s_4_0)).value(),
            u16::try_from(s_4_3).unwrap(),
        ));
        // D s_4_5: cast reint s_4_4 -> u8
        let s_4_5: bool = ((s_4_4.value()) != 0);
        // C s_4_6: const #0s : i
        let s_4_6: i128 = 0;
        // C s_4_7: const #0u : u64
        let s_4_7: u64 = 0;
        // D s_4_8: cast zx s_4_5 -> u64
        let s_4_8: u64 = (s_4_5 as u64);
        // C s_4_9: const #1u : u64
        let s_4_9: u64 = 1;
        // D s_4_10: and s_4_8 s_4_9
        let s_4_10: u64 = ((s_4_8) & (s_4_9));
        // D s_4_11: cmp-eq s_4_10 s_4_9
        let s_4_11: bool = ((s_4_10) == (s_4_9));
        // D s_4_12: lsl s_4_8 s_4_6
        let s_4_12: u64 = s_4_8 << s_4_6;
        // D s_4_13: or s_4_7 s_4_12
        let s_4_13: u64 = ((s_4_7) | (s_4_12));
        // D s_4_14: cmpl s_4_12
        let s_4_14: u64 = !s_4_12;
        // D s_4_15: and s_4_7 s_4_14
        let s_4_15: u64 = ((s_4_7) & (s_4_14));
        // D s_4_16: select s_4_11 s_4_13 s_4_15
        let s_4_16: u64 = if s_4_11 { s_4_13 } else { s_4_15 };
        // D s_4_17: cast trunc s_4_16 -> u8
        let s_4_17: bool = ((s_4_16) != 0);
        // C s_4_18: const #() : ()
        let s_4_18: () = ();
        // S s_4_19: call __UNKNOWN_bit(s_4_18)
        let s_4_19: bool = u__UNKNOWN_bit(state, tracer, s_4_18);
        // D s_4_20: call TargetSecurityState(s_4_17, s_4_19)
        let s_4_20: u32 = TargetSecurityState(state, tracer, s_4_17, s_4_19);
        // D s_4_21: write-var c.6 <= s_4_20
        fn_state.c._6 = s_4_20;
        // D s_4_22: read-var target_el:u8
        let s_4_22: u8 = fn_state.target_el;
        // D s_4_23: write-var c.7 <= s_4_22
        fn_state.c._7 = s_4_22;
        // C s_4_24: const #() : ()
        let s_4_24: () = ();
        // S s_4_25: call EL2Enabled(s_4_24)
        let s_4_25: bool = EL2Enabled(state, tracer, s_4_24);
        // N s_4_26: branch s_4_25 b12 b5
        if s_4_25 {
            return block_12(state, tracer, fn_state);
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
        // D s_5_1: write-var c.4 <= s_5_0
        fn_state.c._4 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #16975u : u32
        let s_6_0: u32 = 16975;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 2u16);
        // C s_6_3: const #448u : u32
        let s_6_3: u32 = 448;
        // D s_6_4: read-reg s_6_3:u8
        let s_6_4: u8 = {
            let value = state.read_register::<u8>(s_6_3 as isize);
            tracer.read_register(s_6_3 as isize, value);
            value
        };
        // D s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 2u16);
        // D s_6_6: cmp-eq s_6_2 s_6_5
        let s_6_6: bool = ((s_6_2) == (s_6_5));
        // N s_6_7: branch s_6_6 b11 b7
        if s_6_6 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var target_el:u8
        let s_7_0: u8 = fn_state.target_el;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 2u16);
        // C s_7_2: const #448u : u32
        let s_7_2: u32 = 448;
        // D s_7_3: read-reg s_7_2:u8
        let s_7_3: u8 = {
            let value = state.read_register::<u8>(s_7_2 as isize);
            tracer.read_register(s_7_2 as isize, value);
            value
        };
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 2u16);
        // D s_7_5: cmp-eq s_7_1 s_7_4
        let s_7_5: bool = ((s_7_1) == (s_7_4));
        // N s_7_6: branch s_7_5 b10 b8
        if s_7_5 {
            return block_10(state, tracer, fn_state);
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
        // D s_8_1: write-var c.3 <= s_8_0
        fn_state.c._3 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var restriction:u32
        let s_9_0: u32 = fn_state.restriction;
        // D s_9_1: write-var c.5 <= s_9_0
        fn_state.c._5 = s_9_0;
        // D s_9_2: read-var c:struct
        let s_9_2: ProductType63dc1b957c45bf6b = fn_state.c;
        // D s_9_3: call RESTRICT_PREDICTIONS(s_9_2)
        let s_9_3: () = RESTRICT_PREDICTIONS(state, tracer, s_9_2);
        // N s_9_4: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var c.3 <= s_10_0
        fn_state.c._3 = s_10_0;
        // C s_10_2: const #8s : i
        let s_10_2: i128 = 8;
        // D s_10_3: read-var val_name:u32
        let s_10_3: u32 = fn_state.val_name;
        // D s_10_4: cast zx s_10_3 -> bv
        let s_10_4: Bits = Bits::new(s_10_3 as u128, 32u16);
        // C s_10_5: const #1u : u64
        let s_10_5: u64 = 1;
        // D s_10_6: bit-extract s_10_4 s_10_2 s_10_5
        let s_10_6: Bits = (Bits::new(
            ((s_10_4) >> (s_10_2)).value(),
            u16::try_from(s_10_5).unwrap(),
        ));
        // D s_10_7: cast reint s_10_6 -> u8
        let s_10_7: bool = ((s_10_6.value()) != 0);
        // C s_10_8: const #0s : i
        let s_10_8: i128 = 0;
        // C s_10_9: const #0u : u64
        let s_10_9: u64 = 0;
        // D s_10_10: cast zx s_10_7 -> u64
        let s_10_10: u64 = (s_10_7 as u64);
        // C s_10_11: const #1u : u64
        let s_10_11: u64 = 1;
        // D s_10_12: and s_10_10 s_10_11
        let s_10_12: u64 = ((s_10_10) & (s_10_11));
        // D s_10_13: cmp-eq s_10_12 s_10_11
        let s_10_13: bool = ((s_10_12) == (s_10_11));
        // D s_10_14: lsl s_10_10 s_10_8
        let s_10_14: u64 = s_10_10 << s_10_8;
        // D s_10_15: or s_10_9 s_10_14
        let s_10_15: u64 = ((s_10_9) | (s_10_14));
        // D s_10_16: cmpl s_10_14
        let s_10_16: u64 = !s_10_14;
        // D s_10_17: and s_10_9 s_10_16
        let s_10_17: u64 = ((s_10_9) & (s_10_16));
        // D s_10_18: select s_10_13 s_10_15 s_10_17
        let s_10_18: u64 = if s_10_13 { s_10_15 } else { s_10_17 };
        // D s_10_19: cast trunc s_10_18 -> u8
        let s_10_19: bool = ((s_10_18) != 0);
        // D s_10_20: cast zx s_10_19 -> bv
        let s_10_20: Bits = Bits::new(s_10_19 as u128, 1u16);
        // C s_10_21: const #1u : u8
        let s_10_21: bool = true;
        // C s_10_22: cast zx s_10_21 -> bv
        let s_10_22: Bits = Bits::new(s_10_21 as u128, 1u16);
        // D s_10_23: cmp-eq s_10_20 s_10_22
        let s_10_23: bool = ((s_10_20) == (s_10_22));
        // D s_10_24: write-var c.0 <= s_10_23
        fn_state.c._0 = s_10_23;
        // C s_10_25: const #0s : i
        let s_10_25: i128 = 0;
        // D s_10_26: read-var val_name:u32
        let s_10_26: u32 = fn_state.val_name;
        // D s_10_27: cast zx s_10_26 -> bv
        let s_10_27: Bits = Bits::new(s_10_26 as u128, 32u16);
        // C s_10_28: const #1s : i64
        let s_10_28: i64 = 1;
        // C s_10_29: cast zx s_10_28 -> i
        let s_10_29: i128 = (i128::try_from(s_10_28).unwrap());
        // C s_10_30: const #7s : i
        let s_10_30: i128 = 7;
        // C s_10_31: add s_10_30 s_10_29
        let s_10_31: i128 = (s_10_30 + s_10_29);
        // D s_10_32: bit-extract s_10_27 s_10_25 s_10_31
        let s_10_32: Bits = (Bits::new(
            ((s_10_27) >> (s_10_25)).value(),
            u16::try_from(s_10_31).unwrap(),
        ));
        // D s_10_33: cast reint s_10_32 -> u8
        let s_10_33: u8 = (s_10_32.value() as u8);
        // C s_10_34: const #16s : i
        let s_10_34: i128 = 16;
        // D s_10_35: cast zx s_10_33 -> bv
        let s_10_35: Bits = Bits::new(s_10_33 as u128, 8u16);
        // D s_10_36: bits-cast zx s_10_35 -> bv length s_10_34
        let s_10_36: Bits = s_10_35.zero_extend(s_10_34);
        // D s_10_37: cast reint s_10_36 -> u16
        let s_10_37: u16 = (s_10_36.value() as u16);
        // D s_10_38: write-var c.2 <= s_10_37
        fn_state.c._2 = s_10_37;
        // N s_10_39: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var c.3 <= s_11_0
        fn_state.c._3 = s_11_0;
        // C s_11_2: const #0u : u8
        let s_11_2: bool = false;
        // D s_11_3: write-var c.0 <= s_11_2
        fn_state.c._0 = s_11_2;
        // C s_11_4: const #() : ()
        let s_11_4: () = ();
        // S s_11_5: call ASID_read(s_11_4)
        let s_11_5: u16 = ASID_read(state, tracer, s_11_4);
        // D s_11_6: write-var c.2 <= s_11_5
        fn_state.c._2 = s_11_5;
        // N s_11_7: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #16975u : u32
        let s_12_0: u32 = 16975;
        // D s_12_1: read-reg s_12_0:u8
        let s_12_1: u8 = {
            let value = state.read_register::<u8>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 2u16);
        // C s_12_3: const #448u : u32
        let s_12_3: u32 = 448;
        // D s_12_4: read-reg s_12_3:u8
        let s_12_4: u8 = {
            let value = state.read_register::<u8>(s_12_3 as isize);
            tracer.read_register(s_12_3 as isize, value);
            value
        };
        // D s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 2u16);
        // D s_12_6: cmp-eq s_12_2 s_12_5
        let s_12_6: bool = ((s_12_2) == (s_12_5));
        // N s_12_7: branch s_12_6 b22 b13
        if s_12_6 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #16975u : u32
        let s_13_0: u32 = 16975;
        // D s_13_1: read-reg s_13_0:u8
        let s_13_1: u8 = {
            let value = state.read_register::<u8>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 2u16);
        // C s_13_3: const #440u : u32
        let s_13_3: u32 = 440;
        // D s_13_4: read-reg s_13_3:u8
        let s_13_4: u8 = {
            let value = state.read_register::<u8>(s_13_3 as isize);
            tracer.read_register(s_13_3 as isize, value);
            value
        };
        // D s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 2u16);
        // D s_13_6: cmp-eq s_13_2 s_13_5
        let s_13_6: bool = ((s_13_2) == (s_13_5));
        // D s_13_7: write-var gs#30491 <= s_13_6
        fn_state.gs_30491 = s_13_6;
        // N s_13_8: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#30491:u8
        let s_14_0: bool = fn_state.gs_30491;
        // N s_14_1: branch s_14_0 b21 b15
        if s_14_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var target_el:u8
        let s_15_0: u8 = fn_state.target_el;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 2u16);
        // C s_15_2: const #448u : u32
        let s_15_2: u32 = 448;
        // D s_15_3: read-reg s_15_2:u8
        let s_15_3: u8 = {
            let value = state.read_register::<u8>(s_15_2 as isize);
            tracer.read_register(s_15_2 as isize, value);
            value
        };
        // D s_15_4: cast zx s_15_3 -> bv
        let s_15_4: Bits = Bits::new(s_15_3 as u128, 2u16);
        // D s_15_5: cmp-eq s_15_1 s_15_4
        let s_15_5: bool = ((s_15_1) == (s_15_4));
        // N s_15_6: branch s_15_5 b20 b16
        if s_15_5 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var target_el:u8
        let s_16_0: u8 = fn_state.target_el;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 2u16);
        // C s_16_2: const #440u : u32
        let s_16_2: u32 = 440;
        // D s_16_3: read-reg s_16_2:u8
        let s_16_3: u8 = {
            let value = state.read_register::<u8>(s_16_2 as isize);
            tracer.read_register(s_16_2 as isize, value);
            value
        };
        // D s_16_4: cast zx s_16_3 -> bv
        let s_16_4: Bits = Bits::new(s_16_3 as u128, 2u16);
        // D s_16_5: cmp-eq s_16_1 s_16_4
        let s_16_5: bool = ((s_16_1) == (s_16_4));
        // D s_16_6: write-var gs#30492 <= s_16_5
        fn_state.gs_30492 = s_16_5;
        // N s_16_7: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#30492:u8
        let s_17_0: bool = fn_state.gs_30492;
        // N s_17_1: branch s_17_0 b19 b18
        if s_17_0 {
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
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var c.4 <= s_18_0
        fn_state.c._4 = s_18_0;
        // N s_18_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var c.4 <= s_19_0
        fn_state.c._4 = s_19_0;
        // C s_19_2: const #27s : i
        let s_19_2: i128 = 27;
        // D s_19_3: read-var val_name:u32
        let s_19_3: u32 = fn_state.val_name;
        // D s_19_4: cast zx s_19_3 -> bv
        let s_19_4: Bits = Bits::new(s_19_3 as u128, 32u16);
        // C s_19_5: const #1u : u64
        let s_19_5: u64 = 1;
        // D s_19_6: bit-extract s_19_4 s_19_2 s_19_5
        let s_19_6: Bits = (Bits::new(
            ((s_19_4) >> (s_19_2)).value(),
            u16::try_from(s_19_5).unwrap(),
        ));
        // D s_19_7: cast reint s_19_6 -> u8
        let s_19_7: bool = ((s_19_6.value()) != 0);
        // C s_19_8: const #0s : i
        let s_19_8: i128 = 0;
        // C s_19_9: const #0u : u64
        let s_19_9: u64 = 0;
        // D s_19_10: cast zx s_19_7 -> u64
        let s_19_10: u64 = (s_19_7 as u64);
        // C s_19_11: const #1u : u64
        let s_19_11: u64 = 1;
        // D s_19_12: and s_19_10 s_19_11
        let s_19_12: u64 = ((s_19_10) & (s_19_11));
        // D s_19_13: cmp-eq s_19_12 s_19_11
        let s_19_13: bool = ((s_19_12) == (s_19_11));
        // D s_19_14: lsl s_19_10 s_19_8
        let s_19_14: u64 = s_19_10 << s_19_8;
        // D s_19_15: or s_19_9 s_19_14
        let s_19_15: u64 = ((s_19_9) | (s_19_14));
        // D s_19_16: cmpl s_19_14
        let s_19_16: u64 = !s_19_14;
        // D s_19_17: and s_19_9 s_19_16
        let s_19_17: u64 = ((s_19_9) & (s_19_16));
        // D s_19_18: select s_19_13 s_19_15 s_19_17
        let s_19_18: u64 = if s_19_13 { s_19_15 } else { s_19_17 };
        // D s_19_19: cast trunc s_19_18 -> u8
        let s_19_19: bool = ((s_19_18) != 0);
        // D s_19_20: cast zx s_19_19 -> bv
        let s_19_20: Bits = Bits::new(s_19_19 as u128, 1u16);
        // C s_19_21: const #1u : u8
        let s_19_21: bool = true;
        // C s_19_22: cast zx s_19_21 -> bv
        let s_19_22: Bits = Bits::new(s_19_21 as u128, 1u16);
        // D s_19_23: cmp-eq s_19_20 s_19_22
        let s_19_23: bool = ((s_19_20) == (s_19_22));
        // D s_19_24: write-var c.1 <= s_19_23
        fn_state.c._1 = s_19_23;
        // C s_19_25: const #16s : i
        let s_19_25: i128 = 16;
        // D s_19_26: read-var val_name:u32
        let s_19_26: u32 = fn_state.val_name;
        // D s_19_27: cast zx s_19_26 -> bv
        let s_19_27: Bits = Bits::new(s_19_26 as u128, 32u16);
        // C s_19_28: const #1s : i64
        let s_19_28: i64 = 1;
        // C s_19_29: cast zx s_19_28 -> i
        let s_19_29: i128 = (i128::try_from(s_19_28).unwrap());
        // C s_19_30: const #7s : i
        let s_19_30: i128 = 7;
        // C s_19_31: add s_19_30 s_19_29
        let s_19_31: i128 = (s_19_30 + s_19_29);
        // D s_19_32: bit-extract s_19_27 s_19_25 s_19_31
        let s_19_32: Bits = (Bits::new(
            ((s_19_27) >> (s_19_25)).value(),
            u16::try_from(s_19_31).unwrap(),
        ));
        // D s_19_33: cast reint s_19_32 -> u8
        let s_19_33: u8 = (s_19_32.value() as u8);
        // C s_19_34: const #16s : i
        let s_19_34: i128 = 16;
        // D s_19_35: cast zx s_19_33 -> bv
        let s_19_35: Bits = Bits::new(s_19_33 as u128, 8u16);
        // D s_19_36: bits-cast zx s_19_35 -> bv length s_19_34
        let s_19_36: Bits = s_19_35.zero_extend(s_19_34);
        // D s_19_37: cast reint s_19_36 -> u16
        let s_19_37: u16 = (s_19_36.value() as u16);
        // D s_19_38: write-var c.8 <= s_19_37
        fn_state.c._8 = s_19_37;
        // N s_19_39: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#30492 <= s_20_0
        fn_state.gs_30492 = s_20_0;
        // N s_20_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var c.4 <= s_21_0
        fn_state.c._4 = s_21_0;
        // C s_21_2: const #0u : u8
        let s_21_2: bool = false;
        // D s_21_3: write-var c.1 <= s_21_2
        fn_state.c._1 = s_21_2;
        // C s_21_4: const #() : ()
        let s_21_4: () = ();
        // S s_21_5: call VMID_read(s_21_4)
        let s_21_5: u16 = VMID_read(state, tracer, s_21_4);
        // D s_21_6: write-var c.8 <= s_21_5
        fn_state.c._8 = s_21_5;
        // N s_21_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#30491 <= s_22_0
        fn_state.gs_30491 = s_22_0;
        // N s_22_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call EndOfInstruction(s_23_0)
        let s_23_1: () = EndOfInstruction(state, tracer, s_23_0);
        // N s_23_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#30475 <= s_24_0
        fn_state.gs_30475 = s_24_0;
        // N s_24_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
