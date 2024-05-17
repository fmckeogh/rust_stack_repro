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
use ThisInstr::*;
use HaveBTIExt::*;
use common::*;
pub fn AArch64_ExecutingBTIInstr<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_6513: (),
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        return_value: bool,
        op2shadow_81: u8,
        instr: u32,
        gs_6522: bool,
        gs_6519: bool,
        gs_6529: bool,
        gs_6513: (),
    }
    let fn_state = FunctionState {
        gs_6513,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveBTIExt(s_0_0)
        let s_0_1: bool = HaveBTIExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b14 b1
        if s_0_2 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call ThisInstr(s_1_0)
        let s_1_1: u32 = ThisInstr(state, tracer, s_1_0);
        // D s_1_2: write-var instr <= s_1_1
        fn_state.instr = s_1_1;
        // C s_1_3: const #22s : i
        let s_1_3: i128 = 22;
        // D s_1_4: read-var instr:u32
        let s_1_4: u32 = fn_state.instr;
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 32u16);
        // C s_1_6: const #1s : i64
        let s_1_6: i64 = 1;
        // C s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // C s_1_8: const #9s : i
        let s_1_8: i128 = 9;
        // C s_1_9: add s_1_8 s_1_7
        let s_1_9: i128 = (s_1_8 + s_1_7);
        // D s_1_10: bit-extract s_1_5 s_1_3 s_1_9
        let s_1_10: Bits = (Bits::new(
            ((s_1_5) >> (s_1_3)).value(),
            u16::try_from(s_1_9).unwrap(),
        ));
        // D s_1_11: cast reint s_1_10 -> u10
        let s_1_11: u16 = (s_1_10.value() as u16);
        // D s_1_12: cast zx s_1_11 -> bv
        let s_1_12: Bits = Bits::new(s_1_11 as u128, 10u16);
        // C s_1_13: const #852u : u10
        let s_1_13: u16 = 852;
        // C s_1_14: cast zx s_1_13 -> bv
        let s_1_14: Bits = Bits::new(s_1_13 as u128, 10u16);
        // D s_1_15: cmp-eq s_1_12 s_1_14
        let s_1_15: bool = ((s_1_12) == (s_1_14));
        // N s_1_16: branch s_1_15 b13 b2
        if s_1_15 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#6519 <= s_2_0
        fn_state.gs_6519 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var gs#6519:u8
        let s_3_0: bool = fn_state.gs_6519;
        // N s_3_1: branch s_3_0 b12 b4
        if s_3_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#6522 <= s_4_0
        fn_state.gs_6522 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var gs#6522:u8
        let s_5_0: bool = fn_state.gs_6522;
        // N s_5_1: branch s_5_0 b8 b6
        if s_5_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var return_value <= s_6_0
        fn_state.return_value = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var return_value:u8
        let s_7_0: bool = fn_state.return_value;
        // N s_7_1: return s_7_0
        return s_7_0;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #8s : i
        let s_8_0: i128 = 8;
        // D s_8_1: read-var instr:u32
        let s_8_1: u32 = fn_state.instr;
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 32u16);
        // C s_8_3: const #1s : i64
        let s_8_3: i64 = 1;
        // C s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // C s_8_5: const #3s : i
        let s_8_5: i128 = 3;
        // C s_8_6: add s_8_5 s_8_4
        let s_8_6: i128 = (s_8_5 + s_8_4);
        // D s_8_7: bit-extract s_8_2 s_8_0 s_8_6
        let s_8_7: Bits = (Bits::new(
            ((s_8_2) >> (s_8_0)).value(),
            u16::try_from(s_8_6).unwrap(),
        ));
        // D s_8_8: cast reint s_8_7 -> u8
        let s_8_8: u8 = (s_8_7.value() as u8);
        // C s_8_9: const #5s : i
        let s_8_9: i128 = 5;
        // D s_8_10: read-var instr:u32
        let s_8_10: u32 = fn_state.instr;
        // D s_8_11: cast zx s_8_10 -> bv
        let s_8_11: Bits = Bits::new(s_8_10 as u128, 32u16);
        // C s_8_12: const #1s : i64
        let s_8_12: i64 = 1;
        // C s_8_13: cast zx s_8_12 -> i
        let s_8_13: i128 = (i128::try_from(s_8_12).unwrap());
        // C s_8_14: const #2s : i
        let s_8_14: i128 = 2;
        // C s_8_15: add s_8_14 s_8_13
        let s_8_15: i128 = (s_8_14 + s_8_13);
        // D s_8_16: bit-extract s_8_11 s_8_9 s_8_15
        let s_8_16: Bits = (Bits::new(
            ((s_8_11) >> (s_8_9)).value(),
            u16::try_from(s_8_15).unwrap(),
        ));
        // D s_8_17: cast reint s_8_16 -> u8
        let s_8_17: u8 = (s_8_16.value() as u8);
        // D s_8_18: write-var op2shadow#81 <= s_8_17
        fn_state.op2shadow_81 = s_8_17;
        // D s_8_19: cast zx s_8_8 -> bv
        let s_8_19: Bits = Bits::new(s_8_8 as u128, 4u16);
        // C s_8_20: const #4u : u8
        let s_8_20: u8 = 4;
        // C s_8_21: cast zx s_8_20 -> bv
        let s_8_21: Bits = Bits::new(s_8_20 as u128, 4u16);
        // D s_8_22: cmp-eq s_8_19 s_8_21
        let s_8_22: bool = ((s_8_19) == (s_8_21));
        // N s_8_23: branch s_8_22 b11 b9
        if s_8_22 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#6529 <= s_9_0
        fn_state.gs_6529 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var gs#6529:u8
        let s_10_0: bool = fn_state.gs_6529;
        // D s_10_1: write-var return_value <= s_10_0
        fn_state.return_value = s_10_0;
        // N s_10_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_11_0: const #0s : i
        let s_11_0: i128 = 0;
        // D s_11_1: read-var op2shadow#81:u8
        let s_11_1: u8 = fn_state.op2shadow_81;
        // D s_11_2: cast zx s_11_1 -> bv
        let s_11_2: Bits = Bits::new(s_11_1 as u128, 3u16);
        // C s_11_3: const #1u : u64
        let s_11_3: u64 = 1;
        // D s_11_4: bit-extract s_11_2 s_11_0 s_11_3
        let s_11_4: Bits = (Bits::new(
            ((s_11_2) >> (s_11_0)).value(),
            u16::try_from(s_11_3).unwrap(),
        ));
        // D s_11_5: cast reint s_11_4 -> u8
        let s_11_5: bool = ((s_11_4.value()) != 0);
        // C s_11_6: const #0s : i
        let s_11_6: i128 = 0;
        // C s_11_7: const #0u : u64
        let s_11_7: u64 = 0;
        // D s_11_8: cast zx s_11_5 -> u64
        let s_11_8: u64 = (s_11_5 as u64);
        // C s_11_9: const #1u : u64
        let s_11_9: u64 = 1;
        // D s_11_10: and s_11_8 s_11_9
        let s_11_10: u64 = ((s_11_8) & (s_11_9));
        // D s_11_11: cmp-eq s_11_10 s_11_9
        let s_11_11: bool = ((s_11_10) == (s_11_9));
        // D s_11_12: lsl s_11_8 s_11_6
        let s_11_12: u64 = s_11_8 << s_11_6;
        // D s_11_13: or s_11_7 s_11_12
        let s_11_13: u64 = ((s_11_7) | (s_11_12));
        // D s_11_14: cmpl s_11_12
        let s_11_14: u64 = !s_11_12;
        // D s_11_15: and s_11_7 s_11_14
        let s_11_15: u64 = ((s_11_7) & (s_11_14));
        // D s_11_16: select s_11_11 s_11_13 s_11_15
        let s_11_16: u64 = if s_11_11 { s_11_13 } else { s_11_15 };
        // D s_11_17: cast trunc s_11_16 -> u8
        let s_11_17: bool = ((s_11_16) != 0);
        // D s_11_18: cast zx s_11_17 -> bv
        let s_11_18: Bits = Bits::new(s_11_17 as u128, 1u16);
        // C s_11_19: const #0u : u8
        let s_11_19: bool = false;
        // C s_11_20: cast zx s_11_19 -> bv
        let s_11_20: Bits = Bits::new(s_11_19 as u128, 1u16);
        // D s_11_21: cmp-eq s_11_18 s_11_20
        let s_11_21: bool = ((s_11_18) == (s_11_20));
        // D s_11_22: write-var gs#6529 <= s_11_21
        fn_state.gs_6529 = s_11_21;
        // N s_11_23: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #0s : i
        let s_12_0: i128 = 0;
        // D s_12_1: read-var instr:u32
        let s_12_1: u32 = fn_state.instr;
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 32u16);
        // C s_12_3: const #1s : i64
        let s_12_3: i64 = 1;
        // C s_12_4: cast zx s_12_3 -> i
        let s_12_4: i128 = (i128::try_from(s_12_3).unwrap());
        // C s_12_5: const #4s : i
        let s_12_5: i128 = 4;
        // C s_12_6: add s_12_5 s_12_4
        let s_12_6: i128 = (s_12_5 + s_12_4);
        // D s_12_7: bit-extract s_12_2 s_12_0 s_12_6
        let s_12_7: Bits = (Bits::new(
            ((s_12_2) >> (s_12_0)).value(),
            u16::try_from(s_12_6).unwrap(),
        ));
        // D s_12_8: cast reint s_12_7 -> u8
        let s_12_8: u8 = (s_12_7.value() as u8);
        // D s_12_9: cast zx s_12_8 -> bv
        let s_12_9: Bits = Bits::new(s_12_8 as u128, 5u16);
        // C s_12_10: const #31u : u8
        let s_12_10: u8 = 31;
        // C s_12_11: cast zx s_12_10 -> bv
        let s_12_11: Bits = Bits::new(s_12_10 as u128, 5u16);
        // D s_12_12: cmp-eq s_12_9 s_12_11
        let s_12_12: bool = ((s_12_9) == (s_12_11));
        // D s_12_13: write-var gs#6522 <= s_12_12
        fn_state.gs_6522 = s_12_12;
        // N s_12_14: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #12s : i
        let s_13_0: i128 = 12;
        // D s_13_1: read-var instr:u32
        let s_13_1: u32 = fn_state.instr;
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 32u16);
        // C s_13_3: const #1s : i64
        let s_13_3: i64 = 1;
        // C s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // C s_13_5: const #9s : i
        let s_13_5: i128 = 9;
        // C s_13_6: add s_13_5 s_13_4
        let s_13_6: i128 = (s_13_5 + s_13_4);
        // D s_13_7: bit-extract s_13_2 s_13_0 s_13_6
        let s_13_7: Bits = (Bits::new(
            ((s_13_2) >> (s_13_0)).value(),
            u16::try_from(s_13_6).unwrap(),
        ));
        // D s_13_8: cast reint s_13_7 -> u10
        let s_13_8: u16 = (s_13_7.value() as u16);
        // D s_13_9: cast zx s_13_8 -> bv
        let s_13_9: Bits = Bits::new(s_13_8 as u128, 10u16);
        // C s_13_10: const #50u : u10
        let s_13_10: u16 = 50;
        // C s_13_11: cast zx s_13_10 -> bv
        let s_13_11: Bits = Bits::new(s_13_10 as u128, 10u16);
        // D s_13_12: cmp-eq s_13_9 s_13_11
        let s_13_12: bool = ((s_13_9) == (s_13_11));
        // D s_13_13: write-var gs#6519 <= s_13_12
        fn_state.gs_6519 = s_13_12;
        // N s_13_14: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var return_value <= s_14_0
        fn_state.return_value = s_14_0;
        // N s_14_2: jump b7
        return block_7(state, tracer, fn_state);
    }
}
