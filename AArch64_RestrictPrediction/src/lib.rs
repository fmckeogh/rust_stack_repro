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
use IsInHost::*;
use HaveRME::*;
use ASID_read::*;
use EndOfInstruction::*;
use TargetSecurityState::*;
use RESTRICT_PREDICTIONS::*;
use VMID_read::*;
use EL2Enabled::*;
use ELIsInHost::*;
use common::*;
pub fn AArch64_RestrictPrediction<T: Tracer>(
    state: &mut State,
    tracer: &T,
    val_name: u64,
    restriction: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_27402: bool,
        gs_27415: bool,
        c: ProductType63dc1b957c45bf6b,
        gs_27395: bool,
        gs_27418: bool,
        target_el: u8,
        gs_27417: bool,
        gs_27401: bool,
        ss: u32,
        gs_27416: bool,
        gs_27403: bool,
        val_name: u64,
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
        // D s_0_1: read-var val_name:u64
        let s_0_1: u64 = fn_state.val_name;
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 64u16);
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
        // N s_0_14: branch s_0_13 b45 b1
        if s_0_13 {
            return block_45(state, tracer, fn_state);
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
        // D s_1_12: write-var gs#27395 <= s_1_11
        fn_state.gs_27395 = s_1_11;
        // N s_1_13: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#27395:u8
        let s_2_0: bool = fn_state.gs_27395;
        // N s_2_1: branch s_2_0 b44 b3
        if s_2_0 {
            return block_44(state, tracer, fn_state);
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
        // D s_4_1: read-var val_name:u64
        let s_4_1: u64 = fn_state.val_name;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 64u16);
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
        // C s_4_18: const #27s : i
        let s_4_18: i128 = 27;
        // D s_4_19: read-var val_name:u64
        let s_4_19: u64 = fn_state.val_name;
        // D s_4_20: cast zx s_4_19 -> bv
        let s_4_20: Bits = Bits::new(s_4_19 as u128, 64u16);
        // C s_4_21: const #1u : u64
        let s_4_21: u64 = 1;
        // D s_4_22: bit-extract s_4_20 s_4_18 s_4_21
        let s_4_22: Bits = (Bits::new(
            ((s_4_20) >> (s_4_18)).value(),
            u16::try_from(s_4_21).unwrap(),
        ));
        // D s_4_23: cast reint s_4_22 -> u8
        let s_4_23: bool = ((s_4_22.value()) != 0);
        // C s_4_24: const #0s : i
        let s_4_24: i128 = 0;
        // C s_4_25: const #0u : u64
        let s_4_25: u64 = 0;
        // D s_4_26: cast zx s_4_23 -> u64
        let s_4_26: u64 = (s_4_23 as u64);
        // C s_4_27: const #1u : u64
        let s_4_27: u64 = 1;
        // D s_4_28: and s_4_26 s_4_27
        let s_4_28: u64 = ((s_4_26) & (s_4_27));
        // D s_4_29: cmp-eq s_4_28 s_4_27
        let s_4_29: bool = ((s_4_28) == (s_4_27));
        // D s_4_30: lsl s_4_26 s_4_24
        let s_4_30: u64 = s_4_26 << s_4_24;
        // D s_4_31: or s_4_25 s_4_30
        let s_4_31: u64 = ((s_4_25) | (s_4_30));
        // D s_4_32: cmpl s_4_30
        let s_4_32: u64 = !s_4_30;
        // D s_4_33: and s_4_25 s_4_32
        let s_4_33: u64 = ((s_4_25) & (s_4_32));
        // D s_4_34: select s_4_29 s_4_31 s_4_33
        let s_4_34: u64 = if s_4_29 { s_4_31 } else { s_4_33 };
        // D s_4_35: cast trunc s_4_34 -> u8
        let s_4_35: bool = ((s_4_34) != 0);
        // D s_4_36: call TargetSecurityState(s_4_17, s_4_35)
        let s_4_36: u32 = TargetSecurityState(state, tracer, s_4_17, s_4_35);
        // D s_4_37: write-var ss <= s_4_36
        fn_state.ss = s_4_36;
        // D s_4_38: read-var ss:u32
        let s_4_38: u32 = fn_state.ss;
        // C s_4_39: const #1u : u32
        let s_4_39: u32 = 1;
        // D s_4_40: cmp-eq s_4_38 s_4_39
        let s_4_40: bool = ((s_4_38) == (s_4_39));
        // N s_4_41: branch s_4_40 b43 b5
        if s_4_40 {
            return block_43(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#27401 <= s_5_0
        fn_state.gs_27401 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#27401:u8
        let s_6_0: bool = fn_state.gs_27401;
        // N s_6_1: branch s_6_0 b42 b7
        if s_6_0 {
            return block_42(state, tracer, fn_state);
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
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call HaveRME(s_8_0)
        let s_8_1: bool = HaveRME(state, tracer, s_8_0);
        // S s_8_2: not s_8_1
        let s_8_2: bool = !s_8_1;
        // N s_8_3: branch s_8_2 b41 b9
        if s_8_2 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#27402 <= s_9_0
        fn_state.gs_27402 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#27402:u8
        let s_10_0: bool = fn_state.gs_27402;
        // N s_10_1: branch s_10_0 b40 b11
        if s_10_0 {
            return block_40(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#27403 <= s_11_0
        fn_state.gs_27403 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#27403:u8
        let s_12_0: bool = fn_state.gs_27403;
        // N s_12_1: branch s_12_0 b39 b13
        if s_12_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var ss:u32
        let s_14_0: u32 = fn_state.ss;
        // D s_14_1: write-var c.6 <= s_14_0
        fn_state.c._6 = s_14_0;
        // D s_14_2: read-var target_el:u8
        let s_14_2: u8 = fn_state.target_el;
        // D s_14_3: write-var c.7 <= s_14_2
        fn_state.c._7 = s_14_2;
        // C s_14_4: const #() : ()
        let s_14_4: () = ();
        // S s_14_5: call EL2Enabled(s_14_4)
        let s_14_5: bool = EL2Enabled(state, tracer, s_14_4);
        // N s_14_6: branch s_14_5 b22 b15
        if s_14_5 {
            return block_22(state, tracer, fn_state);
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
        // D s_15_1: write-var c.4 <= s_15_0
        fn_state.c._4 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #16975u : u32
        let s_16_0: u32 = 16975;
        // D s_16_1: read-reg s_16_0:u8
        let s_16_1: u8 = {
            let value = state.read_register::<u8>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: cast zx s_16_1 -> bv
        let s_16_2: Bits = Bits::new(s_16_1 as u128, 2u16);
        // C s_16_3: const #448u : u32
        let s_16_3: u32 = 448;
        // D s_16_4: read-reg s_16_3:u8
        let s_16_4: u8 = {
            let value = state.read_register::<u8>(s_16_3 as isize);
            tracer.read_register(s_16_3 as isize, value);
            value
        };
        // D s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 2u16);
        // D s_16_6: cmp-eq s_16_2 s_16_5
        let s_16_6: bool = ((s_16_2) == (s_16_5));
        // N s_16_7: branch s_16_6 b21 b17
        if s_16_6 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var target_el:u8
        let s_17_0: u8 = fn_state.target_el;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 2u16);
        // C s_17_2: const #448u : u32
        let s_17_2: u32 = 448;
        // D s_17_3: read-reg s_17_2:u8
        let s_17_3: u8 = {
            let value = state.read_register::<u8>(s_17_2 as isize);
            tracer.read_register(s_17_2 as isize, value);
            value
        };
        // D s_17_4: cast zx s_17_3 -> bv
        let s_17_4: Bits = Bits::new(s_17_3 as u128, 2u16);
        // D s_17_5: cmp-eq s_17_1 s_17_4
        let s_17_5: bool = ((s_17_1) == (s_17_4));
        // N s_17_6: branch s_17_5 b20 b18
        if s_17_5 {
            return block_20(state, tracer, fn_state);
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
        // D s_18_1: write-var c.3 <= s_18_0
        fn_state.c._3 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var restriction:u32
        let s_19_0: u32 = fn_state.restriction;
        // D s_19_1: write-var c.5 <= s_19_0
        fn_state.c._5 = s_19_0;
        // D s_19_2: read-var c:struct
        let s_19_2: ProductType63dc1b957c45bf6b = fn_state.c;
        // D s_19_3: call RESTRICT_PREDICTIONS(s_19_2)
        let s_19_3: () = RESTRICT_PREDICTIONS(state, tracer, s_19_2);
        // N s_19_4: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var c.3 <= s_20_0
        fn_state.c._3 = s_20_0;
        // C s_20_2: const #16s : i
        let s_20_2: i128 = 16;
        // D s_20_3: read-var val_name:u64
        let s_20_3: u64 = fn_state.val_name;
        // D s_20_4: cast zx s_20_3 -> bv
        let s_20_4: Bits = Bits::new(s_20_3 as u128, 64u16);
        // C s_20_5: const #1u : u64
        let s_20_5: u64 = 1;
        // D s_20_6: bit-extract s_20_4 s_20_2 s_20_5
        let s_20_6: Bits = (Bits::new(
            ((s_20_4) >> (s_20_2)).value(),
            u16::try_from(s_20_5).unwrap(),
        ));
        // D s_20_7: cast reint s_20_6 -> u8
        let s_20_7: bool = ((s_20_6.value()) != 0);
        // C s_20_8: const #0s : i
        let s_20_8: i128 = 0;
        // C s_20_9: const #0u : u64
        let s_20_9: u64 = 0;
        // D s_20_10: cast zx s_20_7 -> u64
        let s_20_10: u64 = (s_20_7 as u64);
        // C s_20_11: const #1u : u64
        let s_20_11: u64 = 1;
        // D s_20_12: and s_20_10 s_20_11
        let s_20_12: u64 = ((s_20_10) & (s_20_11));
        // D s_20_13: cmp-eq s_20_12 s_20_11
        let s_20_13: bool = ((s_20_12) == (s_20_11));
        // D s_20_14: lsl s_20_10 s_20_8
        let s_20_14: u64 = s_20_10 << s_20_8;
        // D s_20_15: or s_20_9 s_20_14
        let s_20_15: u64 = ((s_20_9) | (s_20_14));
        // D s_20_16: cmpl s_20_14
        let s_20_16: u64 = !s_20_14;
        // D s_20_17: and s_20_9 s_20_16
        let s_20_17: u64 = ((s_20_9) & (s_20_16));
        // D s_20_18: select s_20_13 s_20_15 s_20_17
        let s_20_18: u64 = if s_20_13 { s_20_15 } else { s_20_17 };
        // D s_20_19: cast trunc s_20_18 -> u8
        let s_20_19: bool = ((s_20_18) != 0);
        // D s_20_20: cast zx s_20_19 -> bv
        let s_20_20: Bits = Bits::new(s_20_19 as u128, 1u16);
        // C s_20_21: const #1u : u8
        let s_20_21: bool = true;
        // C s_20_22: cast zx s_20_21 -> bv
        let s_20_22: Bits = Bits::new(s_20_21 as u128, 1u16);
        // D s_20_23: cmp-eq s_20_20 s_20_22
        let s_20_23: bool = ((s_20_20) == (s_20_22));
        // D s_20_24: write-var c.0 <= s_20_23
        fn_state.c._0 = s_20_23;
        // C s_20_25: const #0s : i
        let s_20_25: i128 = 0;
        // D s_20_26: read-var val_name:u64
        let s_20_26: u64 = fn_state.val_name;
        // D s_20_27: cast zx s_20_26 -> bv
        let s_20_27: Bits = Bits::new(s_20_26 as u128, 64u16);
        // C s_20_28: const #1s : i64
        let s_20_28: i64 = 1;
        // C s_20_29: cast zx s_20_28 -> i
        let s_20_29: i128 = (i128::try_from(s_20_28).unwrap());
        // C s_20_30: const #15s : i
        let s_20_30: i128 = 15;
        // C s_20_31: add s_20_30 s_20_29
        let s_20_31: i128 = (s_20_30 + s_20_29);
        // D s_20_32: bit-extract s_20_27 s_20_25 s_20_31
        let s_20_32: Bits = (Bits::new(
            ((s_20_27) >> (s_20_25)).value(),
            u16::try_from(s_20_31).unwrap(),
        ));
        // D s_20_33: cast reint s_20_32 -> u16
        let s_20_33: u16 = (s_20_32.value() as u16);
        // D s_20_34: write-var c.2 <= s_20_33
        fn_state.c._2 = s_20_33;
        // N s_20_35: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var c.3 <= s_21_0
        fn_state.c._3 = s_21_0;
        // C s_21_2: const #0u : u8
        let s_21_2: bool = false;
        // D s_21_3: write-var c.0 <= s_21_2
        fn_state.c._0 = s_21_2;
        // C s_21_4: const #() : ()
        let s_21_4: () = ();
        // S s_21_5: call ASID_read(s_21_4)
        let s_21_5: u16 = ASID_read(state, tracer, s_21_4);
        // D s_21_6: write-var c.2 <= s_21_5
        fn_state.c._2 = s_21_5;
        // N s_21_7: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #16975u : u32
        let s_22_0: u32 = 16975;
        // D s_22_1: read-reg s_22_0:u8
        let s_22_1: u8 = {
            let value = state.read_register::<u8>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: cast zx s_22_1 -> bv
        let s_22_2: Bits = Bits::new(s_22_1 as u128, 2u16);
        // C s_22_3: const #448u : u32
        let s_22_3: u32 = 448;
        // D s_22_4: read-reg s_22_3:u8
        let s_22_4: u8 = {
            let value = state.read_register::<u8>(s_22_3 as isize);
            tracer.read_register(s_22_3 as isize, value);
            value
        };
        // D s_22_5: cast zx s_22_4 -> bv
        let s_22_5: Bits = Bits::new(s_22_4 as u128, 2u16);
        // D s_22_6: cmp-eq s_22_2 s_22_5
        let s_22_6: bool = ((s_22_2) == (s_22_5));
        // N s_22_7: branch s_22_6 b38 b23
        if s_22_6 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#27415 <= s_23_0
        fn_state.gs_27415 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#27415:u8
        let s_24_0: bool = fn_state.gs_27415;
        // N s_24_1: branch s_24_0 b37 b25
        if s_24_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #16975u : u32
        let s_25_0: u32 = 16975;
        // D s_25_1: read-reg s_25_0:u8
        let s_25_1: u8 = {
            let value = state.read_register::<u8>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: cast zx s_25_1 -> bv
        let s_25_2: Bits = Bits::new(s_25_1 as u128, 2u16);
        // C s_25_3: const #440u : u32
        let s_25_3: u32 = 440;
        // D s_25_4: read-reg s_25_3:u8
        let s_25_4: u8 = {
            let value = state.read_register::<u8>(s_25_3 as isize);
            tracer.read_register(s_25_3 as isize, value);
            value
        };
        // D s_25_5: cast zx s_25_4 -> bv
        let s_25_5: Bits = Bits::new(s_25_4 as u128, 2u16);
        // D s_25_6: cmp-eq s_25_2 s_25_5
        let s_25_6: bool = ((s_25_2) == (s_25_5));
        // D s_25_7: write-var gs#27416 <= s_25_6
        fn_state.gs_27416 = s_25_6;
        // N s_25_8: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#27416:u8
        let s_26_0: bool = fn_state.gs_27416;
        // N s_26_1: branch s_26_0 b36 b27
        if s_26_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var target_el:u8
        let s_27_0: u8 = fn_state.target_el;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 2u16);
        // C s_27_2: const #448u : u32
        let s_27_2: u32 = 448;
        // D s_27_3: read-reg s_27_2:u8
        let s_27_3: u8 = {
            let value = state.read_register::<u8>(s_27_2 as isize);
            tracer.read_register(s_27_2 as isize, value);
            value
        };
        // D s_27_4: cast zx s_27_3 -> bv
        let s_27_4: Bits = Bits::new(s_27_3 as u128, 2u16);
        // D s_27_5: cmp-eq s_27_1 s_27_4
        let s_27_5: bool = ((s_27_1) == (s_27_4));
        // N s_27_6: branch s_27_5 b35 b28
        if s_27_5 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#27417 <= s_28_0
        fn_state.gs_27417 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#27417:u8
        let s_29_0: bool = fn_state.gs_27417;
        // N s_29_1: branch s_29_0 b34 b30
        if s_29_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var target_el:u8
        let s_30_0: u8 = fn_state.target_el;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 2u16);
        // C s_30_2: const #440u : u32
        let s_30_2: u32 = 440;
        // D s_30_3: read-reg s_30_2:u8
        let s_30_3: u8 = {
            let value = state.read_register::<u8>(s_30_2 as isize);
            tracer.read_register(s_30_2 as isize, value);
            value
        };
        // D s_30_4: cast zx s_30_3 -> bv
        let s_30_4: Bits = Bits::new(s_30_3 as u128, 2u16);
        // D s_30_5: cmp-eq s_30_1 s_30_4
        let s_30_5: bool = ((s_30_1) == (s_30_4));
        // D s_30_6: write-var gs#27418 <= s_30_5
        fn_state.gs_27418 = s_30_5;
        // N s_30_7: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#27418:u8
        let s_31_0: bool = fn_state.gs_27418;
        // N s_31_1: branch s_31_0 b33 b32
        if s_31_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var c.4 <= s_32_0
        fn_state.c._4 = s_32_0;
        // N s_32_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #1u : u8
        let s_33_0: bool = true;
        // D s_33_1: write-var c.4 <= s_33_0
        fn_state.c._4 = s_33_0;
        // C s_33_2: const #48s : i
        let s_33_2: i128 = 48;
        // D s_33_3: read-var val_name:u64
        let s_33_3: u64 = fn_state.val_name;
        // D s_33_4: cast zx s_33_3 -> bv
        let s_33_4: Bits = Bits::new(s_33_3 as u128, 64u16);
        // C s_33_5: const #1u : u64
        let s_33_5: u64 = 1;
        // D s_33_6: bit-extract s_33_4 s_33_2 s_33_5
        let s_33_6: Bits = (Bits::new(
            ((s_33_4) >> (s_33_2)).value(),
            u16::try_from(s_33_5).unwrap(),
        ));
        // D s_33_7: cast reint s_33_6 -> u8
        let s_33_7: bool = ((s_33_6.value()) != 0);
        // C s_33_8: const #0s : i
        let s_33_8: i128 = 0;
        // C s_33_9: const #0u : u64
        let s_33_9: u64 = 0;
        // D s_33_10: cast zx s_33_7 -> u64
        let s_33_10: u64 = (s_33_7 as u64);
        // C s_33_11: const #1u : u64
        let s_33_11: u64 = 1;
        // D s_33_12: and s_33_10 s_33_11
        let s_33_12: u64 = ((s_33_10) & (s_33_11));
        // D s_33_13: cmp-eq s_33_12 s_33_11
        let s_33_13: bool = ((s_33_12) == (s_33_11));
        // D s_33_14: lsl s_33_10 s_33_8
        let s_33_14: u64 = s_33_10 << s_33_8;
        // D s_33_15: or s_33_9 s_33_14
        let s_33_15: u64 = ((s_33_9) | (s_33_14));
        // D s_33_16: cmpl s_33_14
        let s_33_16: u64 = !s_33_14;
        // D s_33_17: and s_33_9 s_33_16
        let s_33_17: u64 = ((s_33_9) & (s_33_16));
        // D s_33_18: select s_33_13 s_33_15 s_33_17
        let s_33_18: u64 = if s_33_13 { s_33_15 } else { s_33_17 };
        // D s_33_19: cast trunc s_33_18 -> u8
        let s_33_19: bool = ((s_33_18) != 0);
        // D s_33_20: cast zx s_33_19 -> bv
        let s_33_20: Bits = Bits::new(s_33_19 as u128, 1u16);
        // C s_33_21: const #1u : u8
        let s_33_21: bool = true;
        // C s_33_22: cast zx s_33_21 -> bv
        let s_33_22: Bits = Bits::new(s_33_21 as u128, 1u16);
        // D s_33_23: cmp-eq s_33_20 s_33_22
        let s_33_23: bool = ((s_33_20) == (s_33_22));
        // D s_33_24: write-var c.1 <= s_33_23
        fn_state.c._1 = s_33_23;
        // C s_33_25: const #32s : i
        let s_33_25: i128 = 32;
        // D s_33_26: read-var val_name:u64
        let s_33_26: u64 = fn_state.val_name;
        // D s_33_27: cast zx s_33_26 -> bv
        let s_33_27: Bits = Bits::new(s_33_26 as u128, 64u16);
        // C s_33_28: const #1s : i64
        let s_33_28: i64 = 1;
        // C s_33_29: cast zx s_33_28 -> i
        let s_33_29: i128 = (i128::try_from(s_33_28).unwrap());
        // C s_33_30: const #15s : i
        let s_33_30: i128 = 15;
        // C s_33_31: add s_33_30 s_33_29
        let s_33_31: i128 = (s_33_30 + s_33_29);
        // D s_33_32: bit-extract s_33_27 s_33_25 s_33_31
        let s_33_32: Bits = (Bits::new(
            ((s_33_27) >> (s_33_25)).value(),
            u16::try_from(s_33_31).unwrap(),
        ));
        // D s_33_33: cast reint s_33_32 -> u16
        let s_33_33: u16 = (s_33_32.value() as u16);
        // D s_33_34: write-var c.8 <= s_33_33
        fn_state.c._8 = s_33_33;
        // N s_33_35: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // D s_34_1: write-var gs#27418 <= s_34_0
        fn_state.gs_27418 = s_34_0;
        // N s_34_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var target_el:u8
        let s_35_0: u8 = fn_state.target_el;
        // D s_35_1: call ELIsInHost(s_35_0)
        let s_35_1: bool = ELIsInHost(state, tracer, s_35_0);
        // D s_35_2: not s_35_1
        let s_35_2: bool = !s_35_1;
        // D s_35_3: write-var gs#27417 <= s_35_2
        fn_state.gs_27417 = s_35_2;
        // N s_35_4: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #1u : u8
        let s_36_0: bool = true;
        // D s_36_1: write-var c.4 <= s_36_0
        fn_state.c._4 = s_36_0;
        // C s_36_2: const #0u : u8
        let s_36_2: bool = false;
        // D s_36_3: write-var c.1 <= s_36_2
        fn_state.c._1 = s_36_2;
        // C s_36_4: const #() : ()
        let s_36_4: () = ();
        // S s_36_5: call VMID_read(s_36_4)
        let s_36_5: u16 = VMID_read(state, tracer, s_36_4);
        // D s_36_6: write-var c.8 <= s_36_5
        fn_state.c._8 = s_36_5;
        // N s_36_7: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #1u : u8
        let s_37_0: bool = true;
        // D s_37_1: write-var gs#27416 <= s_37_0
        fn_state.gs_27416 = s_37_0;
        // N s_37_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #() : ()
        let s_38_0: () = ();
        // S s_38_1: call IsInHost(s_38_0)
        let s_38_1: bool = IsInHost(state, tracer, s_38_0);
        // S s_38_2: not s_38_1
        let s_38_2: bool = !s_38_1;
        // D s_38_3: write-var gs#27415 <= s_38_2
        fn_state.gs_27415 = s_38_2;
        // N s_38_4: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #() : ()
        let s_39_0: () = ();
        // S s_39_1: call EndOfInstruction(s_39_0)
        let s_39_1: () = EndOfInstruction(state, tracer, s_39_0);
        // N s_39_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var ss:u32
        let s_40_0: u32 = fn_state.ss;
        // C s_40_1: const #3u : u32
        let s_40_1: u32 = 3;
        // D s_40_2: cmp-eq s_40_0 s_40_1
        let s_40_2: bool = ((s_40_0) == (s_40_1));
        // D s_40_3: write-var gs#27403 <= s_40_2
        fn_state.gs_27403 = s_40_2;
        // N s_40_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var target_el:u8
        let s_41_0: u8 = fn_state.target_el;
        // D s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 2u16);
        // C s_41_2: const #424u : u32
        let s_41_2: u32 = 424;
        // D s_41_3: read-reg s_41_2:u8
        let s_41_3: u8 = {
            let value = state.read_register::<u8>(s_41_2 as isize);
            tracer.read_register(s_41_2 as isize, value);
            value
        };
        // D s_41_4: cast zx s_41_3 -> bv
        let s_41_4: Bits = Bits::new(s_41_3 as u128, 2u16);
        // D s_41_5: cmp-eq s_41_1 s_41_4
        let s_41_5: bool = ((s_41_1) == (s_41_4));
        // D s_41_6: write-var gs#27402 <= s_41_5
        fn_state.gs_27402 = s_41_5;
        // N s_41_7: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call EndOfInstruction(s_42_0)
        let s_42_1: () = EndOfInstruction(state, tracer, s_42_0);
        // N s_42_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var target_el:u8
        let s_43_0: u8 = fn_state.target_el;
        // D s_43_1: cast zx s_43_0 -> bv
        let s_43_1: Bits = Bits::new(s_43_0 as u128, 2u16);
        // C s_43_2: const #424u : u32
        let s_43_2: u32 = 424;
        // D s_43_3: read-reg s_43_2:u8
        let s_43_3: u8 = {
            let value = state.read_register::<u8>(s_43_2 as isize);
            tracer.read_register(s_43_2 as isize, value);
            value
        };
        // D s_43_4: cast zx s_43_3 -> bv
        let s_43_4: Bits = Bits::new(s_43_3 as u128, 2u16);
        // D s_43_5: cmp-ne s_43_1 s_43_4
        let s_43_5: bool = ((s_43_1) != (s_43_4));
        // D s_43_6: write-var gs#27401 <= s_43_5
        fn_state.gs_27401 = s_43_5;
        // N s_43_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #() : ()
        let s_44_0: () = ();
        // S s_44_1: call EndOfInstruction(s_44_0)
        let s_44_1: () = EndOfInstruction(state, tracer, s_44_0);
        // N s_44_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #1u : u8
        let s_45_0: bool = true;
        // D s_45_1: write-var gs#27395 <= s_45_0
        fn_state.gs_27395 = s_45_0;
        // N s_45_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
